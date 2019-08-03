use crate::brush::gradient::stops::GradientStopCollection;
use crate::brush::IBrush;
use crate::render_target::IRenderTarget;

use std::ptr;

use com_wrapper::ComWrapper;
use math2d::Point2f;
use winapi::um::d2d1::{ID2D1Brush, ID2D1RadialGradientBrush};
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
    pub fn create<'a>(context: &'a dyn IRenderTarget) -> RadialGradientBrushBuilder<'a> {
        RadialGradientBrushBuilder::new(context)
    }

    pub fn center(&self) -> Point2f {
        unsafe { self.ptr.GetCenter() }.into()
    }

    pub fn gradient_origin_offset(&self) -> Point2f {
        unsafe { self.ptr.GetGradientOriginOffset() }.into()
    }

    /// Retrieves the `GradientStopCollection` associated with this linear gradient brush.
    pub fn gradient_stop_collection(&self) -> GradientStopCollection {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetGradientStopCollection(&mut ptr);
            GradientStopCollection::from_raw(ptr)
        }
    }

    pub fn radius_x(&self) -> f32 {
        unsafe { self.ptr.GetRadiusX() }
    }

    pub fn radius_y(&self) -> f32 {
        unsafe { self.ptr.GetRadiusY() }
    }
}

unsafe impl IBrush for RadialGradientBrush {
    unsafe fn raw_brush(&self) -> &ID2D1Brush {
        &self.ptr
    }
}
