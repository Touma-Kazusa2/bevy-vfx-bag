use std::marker::PhantomData;

use bevy::{
    asset::{Asset, Handle},
    core_pipeline::{
        core_3d::graph::Core3d, fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::{
        query::QueryItem,
        system::{
            lifetimeless::{SRes, SResMut},
            SystemParamItem,
        },
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        globals::{GlobalsBuffer, GlobalsUniform},
        render_asset::{PrepareAssetError, RenderAsset, RenderAssetPlugin, RenderAssets},
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
        RenderApp,
    },
};

///TODO
pub trait PostProcess: Component + Clone + ExtractComponent + Default // + WriteInto + ShaderType
{
    ///TODO
    type BindGroup: AsBindGroup + Asset + Clone + GetShaderDefs;
    ///TODO
    type Label: RenderLabel + Default;
    ///TODO
    fn shader_path() -> String;
    ///TODO
    fn handle(&self) -> Handle<Self::BindGroup>;
    ///TODO
    fn init(_app: &mut App) {}
}

///TODO
pub struct PreparedPostProcessBindGroup<T: PostProcess> {
    _bindings: Vec<(u32, OwnedBindingResource)>,
    bind_group: BindGroup,
    //key: <<T as PostProcess>::BindGroup as AsBindGroup>::Data,
    pipeline_id: CachedRenderPipelineId,
    layouts: Vec<BindGroupLayout>,
    sampler: Sampler,
    //pub properties: MaterialProperties,
    _marker: PhantomData<T>,
}

///TODO
pub trait GetShaderDefs {
    ///TODO
    fn shader_defs(&self) -> Vec<ShaderDefVal> {
        vec![]
    }
}

impl<T: PostProcess> RenderAsset for PreparedPostProcessBindGroup<T> {
    type SourceAsset = <T as PostProcess>::BindGroup;

    type Param = (
        SRes<RenderDevice>,
        SResMut<PipelineCache>,
        SRes<AssetServer>,
        <<T as PostProcess>::BindGroup as AsBindGroup>::Param,
    );

    fn prepare_asset(
        bind_group_res: Self::SourceAsset,
        (render_device, ref mut pipeline_cache, asset_server, ref mut param): &mut SystemParamItem<
            Self::Param,
        >,
    ) -> Result<Self, PrepareAssetError<Self::SourceAsset>> {
        info!("Preparing post process bind group");
        let bind_group_layout = Self::SourceAsset::bind_group_layout(render_device);
        let shared_layout = render_device.create_bind_group_layout(
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

        match bind_group_res.as_bind_group(&bind_group_layout, render_device, param) {
            Ok(prepared) => {
                let layouts = vec![shared_layout, bind_group_layout];
                let shader = asset_server.load(T::shader_path());
                let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("post_process_pipeline".into()),
                    layout: layouts.clone(),
                    // This will setup a fullscreen triangle for the vertex state
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: bind_group_res.shader_defs(),
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
                let sampler = render_device.create_sampler(&SamplerDescriptor::default());
                Ok(Self {
                    _bindings: prepared.bindings,
                    bind_group: prepared.bind_group,
                    //key: prepared.data,
                    pipeline_id,
                    layouts,
                    sampler,
                    _marker: PhantomData,
                })
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                Err(PrepareAssetError::RetryNextUpdate(bind_group_res))
            }
            Err(other) => Err(PrepareAssetError::AsBindGroupError(other)),
        }
    }
}

///TODO
#[derive(Default)]
pub struct PostProcessNode<T: PostProcess>(PhantomData<fn() -> T>);

// The ViewNode trait is required by the ViewNodeRunner
impl<T: PostProcess> ViewNode for PostProcessNode<T> {
    type ViewQuery = (&'static ViewTarget, &'static T);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, component): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let bind_group = world.resource::<RenderAssets<PreparedPostProcessBindGroup<T>>>();
        let prepared_post_process_bind_group =
            bind_group.get(&component.handle()).unwrap_or_else(|| {
                panic!(
                    "Failed to get prepared post process bind group for handle: {:?}",
                    component.handle()
                )
            });
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache
        let Some(pipeline) =
            pipeline_cache.get_render_pipeline(prepared_post_process_bind_group.pipeline_id)
        else {
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
            &prepared_post_process_bind_group.layouts[0],
            // It's important for this to match the BindGroupLayout defined in the PostProcessPipeline
            &BindGroupEntries::sequential((
                // Make sure to use the source view
                post_process.source,
                // Use the sampler created for the pipeline
                &prepared_post_process_bind_group.sampler,
                // Set the settings binding
                globals,
            )),
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

        render_pass.set_bind_group(1, &prepared_post_process_bind_group.bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

///TODO
pub struct PostProcessPlugin<T: PostProcess>(PhantomData<fn() -> T>);

impl<T: PostProcess> Default for PostProcessPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: PostProcess> Plugin for PostProcessPlugin<T> {
    fn build(&self, app: &mut App) {
        T::init(app);
        app.init_asset::<T::BindGroup>().add_plugins((
            ExtractComponentPlugin::<T>::default(),
            RenderAssetPlugin::<PreparedPostProcessBindGroup<T>>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // .add_systems(
            //     Render,
            //     post_process_shader_def_system::<T>.in_set(RenderSet::Queue),
            // )
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode<T>>>(
                Core3d,
                T::Label::default(),
            );
    }

    // fn finish(&self, app: &mut App) {
    // app.init_resource::<PostProcessImage<T>>()
    //     .insert_resource(PostProcessShaderDef::<T>::default());

    // let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
    //     return;
    // };

    // render_app.init_resource::<PostProcessPipeline<T>>();
    // }
}
