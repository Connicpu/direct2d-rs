use crate::enums::{BitmapInterpolationMode, ExtendMode};
use crate::render_target::RenderTarget;

use checked_enum::UncheckedEnum;
use winapi::um::d2d1::ID2D1BitmapBrush;
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
    #[inline]
    pub fn create<'a>(context: &'a RenderTarget) -> BitmapBrushBuilder<'a> {
        BitmapBrushBuilder::new(context)
    }

    #[inline]
    pub fn extend_mode_x(&self) -> UncheckedEnum<ExtendMode> {
        unsafe { self.ptr.GetExtendModeX().into() }
    }

    #[inline]
    pub fn extend_mode_y(&self) -> UncheckedEnum<ExtendMode> {
        unsafe { self.ptr.GetExtendModeY().into() }
    }

    #[inline]
    pub fn interpolation_mode(&self) -> UncheckedEnum<BitmapInterpolationMode> {
        unsafe { self.ptr.GetInterpolationMode().into() }
    }
}

impl std::ops::Deref for BitmapBrush {
    type Target = crate::brush::Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

