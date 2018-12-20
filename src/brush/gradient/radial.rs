use crate::brush::gradient::stops::GradientStopCollection;
use crate::render_target::RenderTarget;
use math2d::Point2f;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::um::d2d1::ID2D1RadialGradientBrush;
use wio::com::ComPtr;

pub use self::builder::RadialGradientBrushBuilder;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Paints an area with a linear gradient.
pub struct RadialGradientBrush {
    ptr: ComPtr<ID2D1RadialGradientBrush>,
}

impl RadialGradientBrush {
    #[inline]
    pub fn create<'a>(context: &'a RenderTarget) -> RadialGradientBrushBuilder<'a> {
        RadialGradientBrushBuilder::new(context)
    }

    #[inline]
    pub fn center(&self) -> Point2f {
        unsafe { self.ptr.GetCenter() }.into()
    }

    #[inline]
    pub fn gradient_origin_offset(&self) -> Point2f {
        unsafe { self.ptr.GetGradientOriginOffset() }.into()
    }

    /// Retrieves the `GradientStopCollection` associated with this linear gradient brush.
    #[inline]
    pub fn gradient_stop_collection(&self) -> GradientStopCollection {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetGradientStopCollection(&mut ptr);
            GradientStopCollection::from_raw(ptr)
        }
    }

    #[inline]
    pub fn radius_x(&self) -> f32 {
        unsafe { self.ptr.GetRadiusX() }
    }

    #[inline]
    pub fn radius_y(&self) -> f32 {
        unsafe { self.ptr.GetRadiusY() }
    }
}

impl std::ops::Deref for RadialGradientBrush {
    type Target = crate::brush::Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

