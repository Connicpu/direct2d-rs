use crate::brush::IBrush;
use crate::enums::{AntialiasMode, LayerOptions};
use crate::geometry::IGeometry;
use crate::layer::Layer;
use crate::render_target::IRenderTarget;

use com_wrapper::ComWrapper;
use math2d::{Matrix3x2f, Rectf};
use winapi::um::d2d1::D2D1_LAYER_PARAMETERS;

#[must_use]
pub struct LayerBuilder<'a, 'b> {
    rt: &'a mut dyn IRenderTarget,
    layer: &'b Layer,
    bounds: Rectf,
    mask: Option<&'b dyn IGeometry>,
    mask_aa: AntialiasMode,
    mask_tr: Matrix3x2f,
    opacity: f32,
    opacity_brush: Option<&'b dyn IBrush>,
    layer_opts: LayerOptions,
}

impl<'a, 'b> LayerBuilder<'a, 'b> {
    #[inline]
    pub fn create(rt: &'a mut dyn IRenderTarget, layer: &'b Layer) -> Self {
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
    pub fn with_mask(mut self, mask: &'b dyn IGeometry) -> Self {
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

                geometricMask: self
                    .mask
                    .map(|g| g.raw_geom() as *const _ as *mut _)
                    .unwrap_or(std::ptr::null_mut()),
                maskAntialiasMode: self.mask_aa as u32,
                maskTransform: self.mask_tr.into(),

                opacity: self.opacity,
                opacityBrush: self
                    .opacity_brush
                    .map(|g| g.raw_brush() as *const _ as *mut _)
                    .unwrap_or(std::ptr::null_mut()),

                layerOptions: self.layer_opts as u32,
            };

            self.rt.raw_rt().PushLayer(&params, self.layer.get_raw());
        }
    }
}
