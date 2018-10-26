use brush::Brush;
use enums::{AntialiasMode, LayerOptions};
use error::D2DResult;
use geometry::{GenericGeometry, Geometry};
use math::{Matrix3x2f, Rectf, Sizef};
use render_target::RenderTarget;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Layer, D2D1_LAYER_PARAMETERS};
use wio::com::ComPtr;

pub struct Layer {
    ptr: ComPtr<ID2D1Layer>,
}

impl Layer {
    #[inline]
    pub fn create<R>(target: &mut R, size: Option<&Sizef>) -> D2DResult<Layer>
    where
        R: RenderTarget,
    {
        let size = match size {
            Some(size) => size as *const _ as *const _,
            None => ptr::null(),
        };

        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = target.rt().CreateLayer(size, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(Layer::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_size(&self) -> Sizef {
        unsafe { self.ptr.GetSize().into() }
    }

    #[inline]
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1Layer>) -> Self {
        Self { ptr }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1Layer {
        self.ptr.as_raw()
    }

    #[inline]
    pub unsafe fn from_raw(raw: *mut ID2D1Layer) -> Self {
        Layer {
            ptr: ComPtr::from_raw(raw),
        }
    }
}

unsafe impl Send for Layer {}
unsafe impl Sync for Layer {}

#[must_use]
pub struct LayerBuilder<'a, 'b, R: RenderTarget + 'a> {
    rt: &'a mut R,
    layer: &'b Layer,
    bounds: Rectf,
    mask: Option<GenericGeometry>,
    mask_aa: AntialiasMode,
    mask_tr: Matrix3x2f,
    opacity: f32,
    opacity_brush: Option<&'b Brush>,
    layer_opts: LayerOptions,
}

impl<'a, 'b, R: RenderTarget + 'a> LayerBuilder<'a, 'b, R> {
    #[inline]
    pub fn create(rt: &'a mut R, layer: &'b Layer) -> Self {
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
    pub fn with_mask(mut self, mask: impl Geometry) -> Self {
        self.mask = Some(mask.to_generic());
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

                geometricMask: match self.mask {
                    Some(mask) => mask.get_ptr(),
                    None => ptr::null_mut(),
                },
                maskAntialiasMode: self.mask_aa as u32,
                maskTransform: self.mask_tr.into(),

                opacity: self.opacity,
                opacityBrush: match self.opacity_brush {
                    Some(brush) => brush.get_ptr(),
                    None => ptr::null_mut(),
                },

                layerOptions: self.layer_opts as u32,
            };

            self.rt.rt().PushLayer(&params, self.layer.get_raw());
        }
    }
}
