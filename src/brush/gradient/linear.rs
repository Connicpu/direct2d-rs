use crate::brush::gradient::stops::GradientStopCollection;
use crate::render_target::RenderTarget;
use math2d::Point2f;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::um::d2d1::ID2D1LinearGradientBrush;
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
    #[inline]
    pub fn create<'a>(context: &'a RenderTarget) -> LinearGradientBrushBuilder<'a> {
        LinearGradientBrushBuilder::new(context)
    }

    #[inline]
    /// Retrieves the starting coordinates of the linear gradient.
    pub fn start_point(&self) -> Point2f {
        unsafe { self.ptr.GetStartPoint().into() }
    }

    #[inline]
    /// Retrieves the ending coordinates of the linear gradient.
    pub fn end_point(&self) -> Point2f {
        unsafe { self.ptr.GetEndPoint().into() }
    }

    #[inline]
    /// Retrieves the `GradientStopCollection` associated with this linear gradient brush.
    pub fn gradient_stop_collection(&self) -> GradientStopCollection {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetGradientStopCollection(&mut ptr);
            GradientStopCollection::from_raw(ptr)
        }
    }
}

impl std::ops::Deref for LinearGradientBrush {
    type Target = crate::brush::Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

