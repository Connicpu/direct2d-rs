use crate::error::D2DResult;
use crate::render_target::RenderTarget;

use math2d::Color;
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
    pub fn new(context: &RenderTarget, color: impl Into<Color>) -> D2DResult<Self> {
        Self::create(context).with_color(color).build()
    }

    #[inline]
    pub fn create<'a>(context: &'a RenderTarget) -> SolidColorBrushBuilder<'a> {
        SolidColorBrushBuilder::new(context)
    }

    #[inline]
    pub fn color(&self) -> Color {
        unsafe { self.ptr.GetColor().into() }
    }
}

impl std::ops::Deref for SolidColorBrush {
    type Target = crate::brush::Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}
