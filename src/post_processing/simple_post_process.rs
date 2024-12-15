use std::marker::PhantomData;

use bevy::{
    core_pipeline::{
        core_3d::graph::Core3d, fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        globals::{GlobalsBuffer, GlobalsUniform},
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
        Render, RenderApp, RenderSet,
    },
};
use encase::internal::WriteInto;
///This trait is used to define a post-processing effect.
pub trait SimplePostProcess:
    Component + Clone + ExtractComponent + Default + WriteInto + ShaderType
{
    ///The label used to identify the post-processing effect.
    type Label: RenderLabel + Default;
    ///The shader path used for the post-processing effect.
    fn shader_path() -> String;
    ///The bind group used to pass data to the shader.
    fn layout(device: &RenderDevice) -> BindGroupLayout {
        device.create_bind_group_layout(
            None,
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (uniform_buffer::<Self>(true),),
            ),
        )
    }
    ///The bind group used to pass data to the shader.
    fn bind_group(
        _world: &World,
        device: &RenderDevice,
        layout: &BindGroupLayout,
        buffer: BindingResource,
        _textures: &TextureInputs,
    ) -> BindGroup {
        device.create_bind_group(None, layout, &BindGroupEntries::sequential((buffer,)))
    }

    ///if you want overwrite this function,maybe you should use trait: PostProcess instead.
    fn textures(_world: &mut World) -> TextureInputs {
        TextureInputs::None
    }

    ///The shader definitions used for the post-processing effect.
    fn shader_defs() -> Vec<ShaderDefVal> {
        vec![]
    }
}

/// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
pub struct PostProcessPipeline<T: SimplePostProcess> {
    layouts: Vec<BindGroupLayout>,
    sampler: Sampler,
    //textures: TextureInputs,
    pub(crate) pipeline_id: CachedRenderPipelineId,
    _marker: PhantomData<T>,
}

///TODO
#[derive(Resource, ExtractResource, Clone)]
pub struct PostProcessImage<T: SimplePostProcess> {
    ///TODO
    pub texture_inputs: TextureInputs,
    _marker: PhantomData<T>,
}

///TODO
#[derive(Resource, ExtractResource, Clone)]
pub struct PostProcessShaderDef<T: SimplePostProcess> {
    shaderdefs: Vec<ShaderDefVal>,
    changed: bool,
    _marker: PhantomData<T>,
}

impl<T: SimplePostProcess> Default for PostProcessShaderDef<T> {
    fn default() -> Self {
        Self {
            shaderdefs: T::shader_defs(),
            changed: false,
            _marker: PhantomData,
        }
    }
}

impl<T: SimplePostProcess> PostProcessShaderDef<T> {
    ///TODO
    pub fn set_shader_defs(&mut self, shaderdefs: Vec<ShaderDefVal>) {
        self.shaderdefs = shaderdefs;
        self.changed = true;
    }

    ///TODO
    pub fn shader_defs(&self) -> &Vec<ShaderDefVal> {
        &self.shaderdefs
    }

    ///TODO
    pub fn changed(&self) -> bool {
        self.changed
    }

    ///TODO
    pub fn clear_changed(&mut self) {
        self.changed = false;
    }
}

fn post_process_shader_def_system<T: SimplePostProcess>(
    mut shader_def: ResMut<PostProcessShaderDef<T>>,
    mut pipeline: ResMut<PostProcessPipeline<T>>,
    pipeline_cache: Res<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    if shader_def.changed() {
        let shader = asset_server.load(T::shader_path());
        let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("post_process_pipeline".into()),
            layout: pipeline.layouts.clone(),
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader,
                shader_defs: shader_def.shader_defs().to_vec(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: false,
        });
        pipeline.pipeline_id = pipeline_id;
        shader_def.clear_changed();
    }
}

///TODO
#[derive(Debug, Clone, Default)]
pub enum TextureInputs {
    ///TODO
    #[default]
    None,
    ///TODO
    Single(Handle<Image>),
    ///TODO
    Multiple(Vec<Handle<Image>>),
}

impl<T: SimplePostProcess> FromWorld for PostProcessImage<T> {
    fn from_world(world: &mut World) -> Self {
        let texture_inputs = T::textures(world);
        Self {
            texture_inputs,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: SimplePostProcess> FromWorld for PostProcessPipeline<T> {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // We need to define the bind group layout used for our pipeline
        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                    // The settings uniform that will control the effect
                    uniform_buffer::<GlobalsUniform>(false),
                ),
            ),
        );

        let layouts = vec![layout, T::layout(render_device)];

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        // Get the shader handle
        let shader = world.load_asset(T::shader_path());

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: layouts.clone(),
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: T::shader_defs(),
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
                zero_initialize_workgroup_memory: false,
            });

        //let textures = T::textures(world);

        Self {
            layouts,
            sampler,
            pipeline_id,
            //textures,
            _marker: std::marker::PhantomData,
        }
    }
}

///TODO

#[derive(Default)]
pub struct PostProcessNode<T: SimplePostProcess>(PhantomData<fn() -> T>);

// The ViewNode trait is required by the ViewNodeRunner
impl<T: SimplePostProcess + WriteInto + ShaderType> ViewNode for PostProcessNode<T> {
    type ViewQuery = (&'static ViewTarget, &'static DynamicUniformIndex<T>);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Get the pipeline resource that contains the global data we need
        // to create the render pipeline
        let post_process_pipeline = world.resource::<PostProcessPipeline<T>>();

        let post_process_image = world.resource::<PostProcessImage<T>>();

        // The pipeline cache is a cache of all previously created pipelines.
        // It is required to avoid creating a new pipeline each frame,
        // which is expensive due to shader compilation.
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache
        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        // Get the settings uniform binding
        let settings_uniforms = world.resource::<ComponentUniforms<T>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let Some(globals) = world.resource::<GlobalsBuffer>().buffer.binding() else {
            return Ok(());
        };

        // This will start a new "post process write", obtaining two texture
        // views from the view target - a `source` and a `destination`.
        // `source` is the "current" main texture and you _must_ write into
        // `destination` because calling `post_process_write()` on the
        // [`ViewTarget`] will internally flip the [`ViewTarget`]'s main
        // texture to the `destination` texture. Failing to do so will cause
        // the current main texture information to be lost.
        let post_process = view_target.post_process_write();

        // The bind_group gets created each frame.
        //
        // Normally, you would create a bind_group in the Queue set,
        // but this doesn't work with the post_process_write().
        // The reason it doesn't work is because each post_process_write will alternate the source/destination.
        // The only way to have the correct source/destination for the bind_group
        // is to make sure you get it during the node execution.
        let shared_bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layouts[0],
            // It's important for this to match the BindGroupLayout defined in the PostProcessPipeline
            &BindGroupEntries::sequential((
                // Make sure to use the source view
                post_process.source,
                // Use the sampler created for the pipeline
                &post_process_pipeline.sampler,
                // Set the settings binding
                globals,
            )),
        );

        let bind_group = T::bind_group(
            world,
            render_context.render_device(),
            &post_process_pipeline.layouts[1],
            settings_binding.clone(),
            &post_process_image.texture_inputs,
        );

        // Begin the render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);

        render_pass.set_bind_group(0, &shared_bind_group, &[]);
        render_pass.set_bind_group(1, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

///TODO
pub struct SimplePostProcessPlugin<T: SimplePostProcess>(PhantomData<fn() -> T>);

impl<T: SimplePostProcess> Default for SimplePostProcessPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: SimplePostProcess + WriteInto + ShaderType> Plugin for SimplePostProcessPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<T>::default(),
            UniformComponentPlugin::<T>::default(),
            ExtractResourcePlugin::<PostProcessImage<T>>::default(),
            ExtractResourcePlugin::<PostProcessShaderDef<T>>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_systems(
                Render,
                post_process_shader_def_system::<T>.in_set(RenderSet::Queue),
            )
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode<T>>>(
                Core3d,
                T::Label::default(),
            );
    }

    fn finish(&self, app: &mut App) {
        app.init_resource::<PostProcessImage<T>>()
            .insert_resource(PostProcessShaderDef::<T>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<PostProcessPipeline<T>>();
    }
}
