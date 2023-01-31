#import bevy_pbr::mesh_view_bindings

struct ShaderTime {
    secs_since_startup: f32,
    dt: f32,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

//@group(1) @binding(2)
//var<uniform> time: ShaderTime;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.width, view.height);

    /*let col_in = textureSample(texture, our_sampler, uv);

    let strength = 8.0;
    
    let x = (uv.x + 4.0 ) * (uv.y + 4.0 ) * (floor((time.secs_since_startup * 10.0)));
    let pwr = (((((x % 13.0) + 1.0) * ((x % 123.0) + 1.0)) % 0.01) - 0.005) * strength;
	let grain = vec3(pwr);

    let col_grain = col_in + (vec4(grain, 1.0));

    let viguv = uv * (1. - uv.yx);
    let vig = viguv.x*viguv.y * 10.0;
    let pvig = pow(vig, 0.25);

    // Sample each color channel with an arbitrary shift
    var output_color = col_grain * pvig;
    //output_color = vec4(pvig);*/

    return textureSample(texture, our_sampler, uv);
}
