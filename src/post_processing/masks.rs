use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*,
        renderer::RenderDevice,
    },
};
use binding_types::uniform_buffer;

use super::simple_post_process::{SimplePostProcess, TextureInputs};

/// This controls the parameters of the effect.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum MaskVariant {
    /// Rounded square type mask.
    ///
    /// One use of this mask is to post-process _other_ effects which might
    /// have artifacts around the edges.
    /// This mask can then attenuate that effect and thus remove the effects of the
    /// artifacts.
    ///
    /// Strength value guidelines for use in [`Mask`]:
    ///
    /// Low end:    3.0 almost loses the square shape.
    /// High end:   100.0 has almost sharp, thin edges.
    Square,

    /// Rounded square type mask, but more oval like a CRT television.
    ///
    /// This effect can be used as a part of a retry-style effect.
    ///
    /// Strength value guidelines for use in [`Mask`]:
    ///
    /// Low end:    3000.0 almost loses the CRT shape.
    /// High end:   500000.0 "inflates" the effect a bit.
    Crt,

    /// Vignette mask.
    ///
    /// This effect can be used to replicate the classic photography
    /// light attenuation seen at the edges of photos.
    ///
    /// Strength value guidelines for use in [`Mask`]:
    ///
    /// Low end:    0.10 gives a very subtle effect.
    /// High end:   1.50 is almost a spotlight in the middle of the screen.
    Vignette,
}

impl From<MaskVariant> for ShaderDefVal {
    fn from(variant: MaskVariant) -> Self {
        match variant {
            MaskVariant::Square => "SQUARE",
            MaskVariant::Crt => "CRT",
            MaskVariant::Vignette => "VIGNETTE",
        }
        .into()
    }
}

impl From<&ShaderDefVal> for MaskVariant {
    fn from(value: &ShaderDefVal) -> Self {
        match value {
            ShaderDefVal::Bool(key, _) => match key.as_str() {
                "SQUARE" => MaskVariant::Square,
                "CRT" => MaskVariant::Crt,
                "VIGNETTE" => MaskVariant::Vignette,
                _ => panic!("Unknown ShaderDefVal key: {}", key),
            },
            ShaderDefVal::Int(key, _) | ShaderDefVal::UInt(key, _) => {
                panic!("ShaderDefVal type not supported for MaskVariant: {}", key)
            }
        }
    }
}

/// A darkening mask on the outer edges of the image.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct Mask {
    /// The strength parameter of the mask in use.
    ///
    /// See [`MaskVariant`] for guidelines on which range of values make sense
    /// for the variant in use.
    ///
    /// Run the masks example to experiment with these values interactively.
    pub strength: f32,

    /// How much the mask is faded: 1.0 - mask has no effect, 0.0 - mask is in full effect
    pub fade: f32,
    // Which [`MaskVariant`] to produce.
    //pub variant: MaskVariant,
}

use std::fmt::Display;
impl Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " strength: {} fade: {}", self.strength, self.fade)
    }
}

impl Mask {
    /// Create a new square mask with a reasonable strength value.
    pub fn square() -> Self {
        Self {
            strength: 20.,
            fade: 0.,
            //variant: MaskVariant::Square,
        }
    }

    /// Create a new CRT mask with a reasonable strength value.
    pub fn crt() -> Self {
        Self {
            strength: 80000.,
            fade: 0.,
            //variant: MaskVariant::Crt,
        }
    }

    /// Create a new vignette mask with a reasonable strength value.
    pub fn vignette() -> Self {
        Self {
            strength: 0.66,
            fade: 0.,
            //variant: MaskVariant::Vignette,
        }
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self::vignette()
    }
}

impl SimplePostProcess for Mask {
    fn shader_path() -> String {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/", "masks.wgsl").into()
    }
    type Label = MaskPostProcessLabel;
    fn layout(device: &RenderDevice) -> BindGroupLayout {
        device.create_bind_group_layout(
            "mask_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (uniform_buffer::<Self>(true),),
            ),
        )
    }
    fn bind_group(
        _world: &World,
        device: &RenderDevice,
        layout: &BindGroupLayout,
        buffer: BindingResource,
        _textures: &TextureInputs,
    ) -> BindGroup {
        device.create_bind_group(
            "mask_bind_group",
            layout,
            &BindGroupEntries::sequential((buffer,)),
        )
    }
    fn shader_defs() -> Vec<ShaderDefVal> {
        vec!["VIGNETTE".into()]
    }
}
///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct MaskPostProcessLabel;
