{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = "github:oxalica/rust-overlay";
    nur.url = github:polygon/nur.nix;
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, rust-overlay, nixpkgs, nur, naersk }:
    let
      systems = [
        "aarch64-linux"
        "i686-linux"
        "x86_64-linux"
      ];
      overlays = [ (import rust-overlay) ];
    in
    builtins.foldl'
      (outputs: system:

        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit overlays system;
          };

          rust-bin = pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default.override {
              targets = [ "wasm32-unknown-unknown" ];
              extensions = [ "rust-src" ];
            });
          naersk-lib = naersk.lib.${system}.override {
            cargo = rust-bin;
            rustc = rust-bin;
          };

          rust-dev-deps = with pkgs; [
            rust-analyzer
            rustfmt
            lldb
            cargo-geiger
            nur.packages.${system}.wasm-server-runner
          ];
          build-deps = with pkgs; [
            pkgconfig
            mold
            clang
            makeWrapper
          ];
          runtime-deps = with pkgs; [
            alsa-lib
            udev
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libxcb
            libGL
            vulkan-loader
            vulkan-headers
          ];
        in
        {
          devShell.${system} =
            let
              all_deps = runtime-deps ++ build-deps ++ rust-dev-deps ++ [ rust-bin ];
            in
            pkgs.mkShell {
              buildInputs = all_deps;
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (all_deps);
              shellHook = ''
                export CARGO_MANIFEST_DIR=$(pwd)
              '';
            };
          packages.${system}.bevy_matrix = naersk-lib.buildPackage
            {
              pname = "bevy_matrix";
              root = ./.;
              buildInputs = runtime-deps;
              nativeBuildInputs = build-deps;
              overrideMain = attrs: {
                fixupPhase = ''
                  wrapProgram $out/bin/bevy_matrix \
                    --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath runtime-deps} \
                    --set CARGO_MANIFEST_DIR $out/share/bevy_matrix
                  mkdir -p $out/share/bevy_matrix
                  cp -a assets $out/share/bevy_matrix'';
                patchPhase = ''
                  sed -i s/\"dynamic\"// Cargo.toml
                '';
              };
            };
          defaultPackage.${system} = self.packages.${system}.bevy_matrix;

        }
      )
      { }
      systems;

}
