use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType};
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, FilterMode, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, SamplerDescriptor,
};
use bevy::render::renderer::RenderContext;
use bevy::render::RenderStage;
use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    ecs::query::QueryItem,
    prelude::*,
    reflect::erased_serde::__private::serde::__private::de::TagContentOtherFieldVisitor,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_resource::{
            BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState,
            MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor,
            SamplerBindingType, ShaderStages, SpecializedRenderPipeline,
            SpecializedRenderPipelines, TextureFormat, TextureSampleType, TextureViewDimension,
            TextureViewId,
        },
        renderer::RenderDevice,
        texture::BevyDefault,
        view::{ExtractedView, ViewTarget},
        RenderApp,
    },
};
use bevy_editor_pls::egui::mutex::Mutex;

#[derive(Component, Clone)]
pub struct MatrixPost {
    pub enabled: bool,
}

impl Default for MatrixPost {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl ExtractComponent for MatrixPost {
    type Query = &'static Self;
    type Filter = With<Camera>;

    fn extract_component(item: QueryItem<Self::Query>) -> Self {
        item.clone()
    }
}

pub const MATRIX_POST_NODE: &str = "matrix_post_node";

pub struct MatrixPostPlugin;
impl Plugin for MatrixPostPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<MatrixPost>::default());

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .init_resource::<MatrixPostPipeline>()
            .init_resource::<SpecializedRenderPipelines<MatrixPostPipeline>>()
            .add_system_to_stage(RenderStage::Prepare, prepare_matrix_post_pipelines);
    }
}

#[derive(Resource)]
pub struct MatrixPostPipeline {
    texture_bind_group: BindGroupLayout,
    frag_shader: Handle<Shader>,
}

impl FromWorld for MatrixPostPipeline {
    fn from_world(render_world: &mut World) -> Self {
        let asset_server = render_world.get_resource::<AssetServer>().unwrap();
        let frag_shader: Handle<Shader> = asset_server.load("shaders/post.wgsl");
        let texture_bind_group = render_world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("matrix_post_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
        Self {
            texture_bind_group,
            frag_shader,
        }
    }
}

#[derive(Component)]
pub struct CameraMatrixPostPipeline {
    pub pipeline_id: CachedRenderPipelineId,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct MatrixPostPipelineKey {
    texture_format: TextureFormat,
}

impl SpecializedRenderPipeline for MatrixPostPipeline {
    type Key = MatrixPostPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("matrix_post".into()),
            layout: Some(vec![self.texture_bind_group.clone()]),
            vertex: fullscreen_shader_vertex_state(),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                shader: self.frag_shader.clone(),
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: key.texture_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
        }
    }
}

pub fn prepare_matrix_post_pipelines(
    mut commands: Commands,
    mut pipeline_cache: ResMut<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<MatrixPostPipeline>>,
    matrix_post_pipeline: Res<MatrixPostPipeline>,
    views: Query<(Entity, &ExtractedView, &MatrixPost)>,
) {
    for (entity, view, post) in &views {
        if !post.enabled {
            continue;
        }

        let pipeline_id = pipelines.specialize(
            &mut pipeline_cache,
            &matrix_post_pipeline,
            MatrixPostPipelineKey {
                texture_format: if view.hdr {
                    ViewTarget::TEXTURE_FORMAT_HDR
                } else {
                    TextureFormat::bevy_default()
                },
            },
        );

        commands
            .entity(entity)
            .insert(CameraMatrixPostPipeline { pipeline_id });
    }
}

pub struct MatrixPostNode {
    query: QueryState<(
        &'static ViewTarget,
        &'static CameraMatrixPostPipeline,
        &'static MatrixPost,
    )>,
    cached_texture_bind_group: Mutex<Option<(TextureViewId, BindGroup)>>,
}

impl MatrixPostNode {
    pub const IN_VIEW: &'static str = "view";

    pub fn new(world: &mut World) -> Self {
        Self {
            query: QueryState::new(world),
            cached_texture_bind_group: Mutex::new(None),
        }
    }
}

impl Node for MatrixPostNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(Self::IN_VIEW, SlotType::Entity)]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world)
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(Self::IN_VIEW)?;
        let pipeline_cache = world.resource::<PipelineCache>();
        let matrix_post_pipeline = world.resource::<MatrixPostPipeline>();

        let (target, pipeline, matrix_post) = match self.query.get_manual(world, view_entity) {
            Ok(result) => result,
            Err(_) => return Ok(()),
        };

        if !matrix_post.enabled {
            return Ok(());
        }

        let pipeline = pipeline_cache
            .get_render_pipeline(pipeline.pipeline_id)
            .unwrap();

        let post_process = target.post_process_write();
        let source = post_process.source;
        let destination = post_process.destination;
        let mut cached_bind_group = self.cached_texture_bind_group.lock();
        let bind_group = match &mut *cached_bind_group {
            Some((id, bind_group)) if source.id() == *id => bind_group,
            cached_bind_group => {
                let sampler = render_context
                    .render_device
                    .create_sampler(&SamplerDescriptor {
                        mipmap_filter: FilterMode::Linear,
                        mag_filter: FilterMode::Linear,
                        min_filter: FilterMode::Linear,
                        ..Default::default()
                    });

                let bind_group =
                    render_context
                        .render_device
                        .create_bind_group(&BindGroupDescriptor {
                            label: None,
                            layout: &matrix_post_pipeline.texture_bind_group,
                            entries: &[
                                BindGroupEntry {
                                    binding: 0,
                                    resource: BindingResource::TextureView(source),
                                },
                                BindGroupEntry {
                                    binding: 1,
                                    resource: BindingResource::Sampler(&sampler),
                                },
                            ],
                        });

                let (_, bind_group) = cached_bind_group.insert((source.id(), bind_group));
                bind_group
            }
        };

        let pass_descriptor = RenderPassDescriptor {
            label: Some("matrix_post_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        };

        let mut render_pass = render_context
            .command_encoder
            .begin_render_pass(&pass_descriptor);

        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
