use crate::brush::Brush;
use crate::enums::{AntialiasMode, LayerOptions};
use crate::geometry::Geometry;
use crate::layer::Layer;
use crate::render_target::RenderTarget;

use com_wrapper::ComWrapper;
use dcommon::helpers::unwrap_opt_com;
use math2d::{Matrix3x2f, Rectf};
use winapi::um::d2d1::D2D1_LAYER_PARAMETERS;

#[must_use]
pub struct LayerBuilder<'a, 'b> {
    rt: &'a mut RenderTarget,
    layer: &'b Layer,
    bounds: Rectf,
    mask: Option<&'b Geometry>,
    mask_aa: AntialiasMode,
    mask_tr: Matrix3x2f,
    opacity: f32,
    opacity_brush: Option<&'b Brush>,
    layer_opts: LayerOptions,
}

impl<'a, 'b> LayerBuilder<'a, 'b> {
    #[inline]
    pub fn create(rt: &'a mut RenderTarget, layer: &'b Layer) -> Self {
        LayerBuilder {
            rt,
            layer,
            bounds: Rectf::INFINITE,
            mask: None,
            mask_aa: AntialiasMode::PerPrimitive,
            mask_tr: Matrix3x2f::IDENTITY,
            opacity: 1.0,
            opacity_brush: None,
            layer_opts: LayerOptions::None,
        }
    }

    #[inline]
    pub fn with_mask(mut self, mask: &'b Geometry) -> Self {
        self.mask = Some(mask);
        self
    }

    #[inline]
    pub fn with_mask_transform(mut self, transform: Matrix3x2f) -> Self {
        self.mask_tr = transform;
        self
    }

    pub fn push(self) {
        unsafe {
            let params = D2D1_LAYER_PARAMETERS {
                contentBounds: self.bounds.into(),

                geometricMask: unwrap_opt_com(self.mask),
                maskAntialiasMode: self.mask_aa as u32,
                maskTransform: self.mask_tr.into(),

                opacity: self.opacity,
                opacityBrush: unwrap_opt_com(self.opacity_brush),

                layerOptions: self.layer_opts as u32,
            };

            self.rt.rt().PushLayer(&params, self.layer.get_raw());
        }
    }
}
