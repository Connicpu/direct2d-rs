use crate::brush::IBrush;
use crate::enums::{BitmapInterpolationMode, ExtendMode};
use crate::render_target::IRenderTarget;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use winapi::um::d2d1::{ID2D1BitmapBrush, ID2D1Brush};
use wio::com::ComPtr;

pub use self::builder::*;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct BitmapBrush {
    ptr: ComPtr<ID2D1BitmapBrush>,
}

impl BitmapBrush {
    pub fn create<'a>(context: &'a dyn IRenderTarget) -> BitmapBrushBuilder<'a> {
        BitmapBrushBuilder::new(context)
    }

    pub fn extend_mode_x(&self) -> UncheckedEnum<ExtendMode> {
        unsafe { self.ptr.GetExtendModeX().into() }
    }

    pub fn extend_mode_y(&self) -> UncheckedEnum<ExtendMode> {
        unsafe { self.ptr.GetExtendModeY().into() }
    }

    pub fn interpolation_mode(&self) -> UncheckedEnum<BitmapInterpolationMode> {
        unsafe { self.ptr.GetInterpolationMode().into() }
    }
}

unsafe impl IBrush for BitmapBrush {
    unsafe fn raw_brush(&self) -> &ID2D1Brush {
        &self.ptr
    }
}
