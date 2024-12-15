use bevy::{
    asset::RenderAssetUsages,
    image::{
        CompressedImageFormats, ImageAddressMode, ImageSampler, ImageSamplerDescriptor, ImageType,
    },
    prelude::*,
    render::{
        extract_component::ExtractComponent, render_asset::RenderAssets, render_graph::RenderLabel,
        render_resource::*, renderer::RenderDevice, texture::GpuImage,
    },
};
use binding_types::{sampler, texture_2d, uniform_buffer};

use super::simple_post_process::{SimplePostProcess, TextureInputs};
///TODO
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct Raindrops {
    /// How quickly the raindrops animate.
    pub speed: f32,

    /// How much the raindrops warp the image.
    pub warping: f32,

    /// How zoomed in the raindrops texture is.
    pub zoom: f32,
}

impl Default for Raindrops {
    fn default() -> Self {
        Self {
            speed: 0.8,
            warping: 0.03,
            zoom: 1.0,
        }
    }
}
use std::fmt::Display;
impl Display for Raindrops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Raindrops speed: {}, warping: {}, zoom: {}",
            self.speed, self.warping, self.zoom
        )
    }
}

impl SimplePostProcess for Raindrops {
    fn shader_path() -> String {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/",
            "raindrops.wgsl"
        )
        .into()
    }
    type Label = RaindropsPostProcessLabel;
    fn layout(device: &RenderDevice) -> BindGroupLayout {
        device.create_bind_group_layout(
            "flip_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<Self>(true),
                ),
            ),
        )
    }
    fn bind_group(
        world: &World,
        device: &RenderDevice,
        layout: &BindGroupLayout,
        buffer: BindingResource,
        textures: &TextureInputs,
    ) -> BindGroup {
        let texture = match textures {
            TextureInputs::Single(texture) => texture,
            _ => panic!("Expected a single texture for raindrops post processing"),
        };

        let gpu_image = world
            .resource::<RenderAssets<GpuImage>>()
            .get(texture)
            .unwrap();
        device.create_bind_group(
            "flip_bind_group",
            layout,
            &BindGroupEntries::sequential((&gpu_image.texture_view, &gpu_image.sampler, buffer)),
        )
    }

    fn textures(world: &mut World) -> TextureInputs {
        let raindrops_sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            label: Some("Raindrops Sampler".into()),
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            address_mode_w: ImageAddressMode::Repeat,
            ..default()
        });

        let image = Image::from_buffer(
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/textures/",
                "raindrops.tga"
            )),
            ImageType::Extension("tga"),
            CompressedImageFormats::NONE,
            false,
            raindrops_sampler,
            RenderAssetUsages::RENDER_WORLD,
        )
        .expect("Should load raindrops successfully");

        let handle = world
            .get_resource_mut::<Assets<Image>>()
            .unwrap()
            .add(image);

        TextureInputs::Single(handle)
    }
}

///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct RaindropsPostProcessLabel;
