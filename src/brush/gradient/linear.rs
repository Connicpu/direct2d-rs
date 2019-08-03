use crate::brush::gradient::stops::GradientStopCollection;
use crate::brush::IBrush;
use crate::render_target::IRenderTarget;

use std::ptr;

use com_wrapper::ComWrapper;
use math2d::Point2f;
use winapi::um::d2d1::{ID2D1Brush, ID2D1LinearGradientBrush};
use wio::com::ComPtr;

pub use self::builder::LinearGradientBrushBuilder;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Paints an area with a linear gradient.
pub struct LinearGradientBrush {
    ptr: ComPtr<ID2D1LinearGradientBrush>,
}

impl LinearGradientBrush {
    pub fn create<'a>(context: &'a dyn IRenderTarget) -> LinearGradientBrushBuilder<'a> {
        LinearGradientBrushBuilder::new(context)
    }

    /// Retrieves the starting coordinates of the linear gradient.
    pub fn start_point(&self) -> Point2f {
        unsafe { self.ptr.GetStartPoint().into() }
    }

    /// Retrieves the ending coordinates of the linear gradient.
    pub fn end_point(&self) -> Point2f {
        unsafe { self.ptr.GetEndPoint().into() }
    }

    /// Retrieves the `GradientStopCollection` associated with this linear gradient brush.
    pub fn gradient_stop_collection(&self) -> GradientStopCollection {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetGradientStopCollection(&mut ptr);
            GradientStopCollection::from_raw(ptr)
        }
    }
}

unsafe impl IBrush for LinearGradientBrush {
    unsafe fn raw_brush(&self) -> &ID2D1Brush {
        &self.ptr
    }
}
