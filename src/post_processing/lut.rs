use bevy::{
    asset::{Handle, RenderAssetUsages},
    image::{CompressedImageFormats, Image, ImageSampler, ImageType},
    prelude::*,
    render::{extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*},
};

use super::post_process::{GetShaderDefs, PostProcess};
///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct LutPostProcessLabel;

///TODO
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LutPostProcessBindGroup {
    ///TODO
    #[texture(0, dimension = "3d")]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl GetShaderDefs for LutPostProcessBindGroup {
    fn shader_defs(&self) -> Vec<ShaderDefVal> {
        vec![]
    }
}

impl PostProcess for Lut {
    fn shader_path() -> String {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/", "lut.wgsl").into()
    }
    type Label = LutPostProcessLabel;

    type BindGroup = LutPostProcessBindGroup;

    fn handle(&self) -> Handle<Self::BindGroup> {
        self.handle.clone()
    }

    fn init(app: &mut App) {
        let mut assets = app.world_mut().resource_mut::<Assets<_>>();

        let image = adapt_image_for_lut_use(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/luts/",
            "neo.png"
        )));
        assets.insert(&LUT_NEO_IMAGE_HANDLE, image);

        let image = adapt_image_for_lut_use(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/luts/",
            "slate.png"
        )));
        assets.insert(&LUT_SLATE_IMAGE_HANDLE, image);

        let image = adapt_image_for_lut_use(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/luts/",
            "arctic.png"
        )));
        assets.insert(&LUT_ARCTIC_IMAGE_HANDLE, image);
    }
}

/// A look-up texture. Maps colors to colors. Useful for colorschemes.
#[derive(Default, Debug, Component, Clone, ExtractComponent)]
pub struct Lut {
    /// The 3D look-up texture
    handle: Handle<LutPostProcessBindGroup>,
    //prepared: bool,
}

impl Lut {
    /// Creates a new LUT component.
    /// The image should be a 64x64x64 3D texture.
    /// See the `make-neutral-lut` example.
    pub fn new(assets: &mut Assets<LutPostProcessBindGroup>, image_handle: Handle<Image>) -> Self {
        let handle = assets.add(LutPostProcessBindGroup {
            texture: image_handle,
        });
        Self {
            handle,
            //prepared: false,
        }
    }

    /// The arctic color scheme LUT.
    pub fn arctic(assets: &mut Assets<LutPostProcessBindGroup>) -> Self {
        Self::new(assets, LUT_ARCTIC_IMAGE_HANDLE)
    }

    /// The neo color scheme LUT.
    pub fn neo(assets: &mut Assets<LutPostProcessBindGroup>) -> Self {
        Self::new(assets, LUT_NEO_IMAGE_HANDLE)
    }

    /// The slate color scheme LUT.
    pub fn slate(assets: &mut Assets<LutPostProcessBindGroup>) -> Self {
        Self::new(assets, LUT_SLATE_IMAGE_HANDLE)
    }
}

const LUT_ARCTIC_IMAGE_HANDLE: Handle<Image> = Handle::weak_from_u128(11514769687270273032);
const LUT_NEO_IMAGE_HANDLE: Handle<Image> = Handle::weak_from_u128(18411885151390434307);
const LUT_SLATE_IMAGE_HANDLE: Handle<Image> = Handle::weak_from_u128(8809687374954616573);

fn adapt_image_for_lut_use(buffer: &[u8]) -> Image {
    let sampler = ImageSampler::Default;
    let mut image = Image::from_buffer(
        buffer,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        false,
        sampler.clone(),
        RenderAssetUsages::RENDER_WORLD,
    )
    .expect("Should load LUT successfully");
    image.texture_descriptor.size = Extent3d {
        width: 64,
        height: 64,
        depth_or_array_layers: 64,
    };
    image.texture_descriptor.dimension = TextureDimension::D3;
    image.texture_descriptor.format = TextureFormat::Rgba8Unorm;

    image.texture_view_descriptor = Some(TextureViewDescriptor {
        label: Some("LUT Texture View"),
        format: Some(TextureFormat::Rgba8Unorm),
        dimension: Some(TextureViewDimension::D3),
        ..default()
    });
    image
}
