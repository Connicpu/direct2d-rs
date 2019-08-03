use crate::brush::IBrush;
use crate::render_target::IRenderTarget;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Color;
use winapi::um::d2d1::ID2D1Brush;
use winapi::um::d2d1::ID2D1SolidColorBrush;
use wio::com::ComPtr;

pub use self::builder::*;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct SolidColorBrush {
    ptr: ComPtr<ID2D1SolidColorBrush>,
}

impl SolidColorBrush {
    pub fn new(
        context: &dyn IRenderTarget,
        color: impl Into<Color>,
    ) -> Result<SolidColorBrush, Error> {
        Self::create(context).with_color(color).build()
    }

    #[inline]
    pub fn create<'a>(context: &'a dyn IRenderTarget) -> SolidColorBrushBuilder<'a> {
        SolidColorBrushBuilder::new(context)
    }

    #[inline]
    pub fn color(&self) -> Color {
        unsafe { self.ptr.GetColor().into() }
    }
}

unsafe impl IBrush for SolidColorBrush {
    unsafe fn raw_brush(&self) -> &ID2D1Brush {
        &self.ptr
    }
}
