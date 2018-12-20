use crate::enums::{CapStyle, DashStyle, LineJoin};
use crate::factory::Factory;

use checked_enum::UncheckedEnum;
use winapi::um::d2d1::ID2D1StrokeStyle;
use wio::com::ComPtr;

pub use self::builder::*;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct StrokeStyle {
    ptr: ComPtr<ID2D1StrokeStyle>,
}

impl StrokeStyle {
    #[inline]
    pub fn create<'a>(factory: &'a Factory) -> StrokeStyleBuilder<'a> {
        StrokeStyleBuilder::new(factory)
    }

    #[inline]
    pub fn start_cap(&self) -> UncheckedEnum<CapStyle> {
        unsafe { self.ptr.GetStartCap().into() }
    }

    #[inline]
    pub fn end_cap(&self) -> UncheckedEnum<CapStyle> {
        unsafe { self.ptr.GetEndCap().into() }
    }

    #[inline]
    pub fn dash_cap(&self) -> UncheckedEnum<CapStyle> {
        unsafe { self.ptr.GetDashCap().into() }
    }

    #[inline]
    pub fn miter_limit(&self) -> f32 {
        unsafe { self.ptr.GetMiterLimit() }
    }

    #[inline]
    pub fn line_join(&self) -> UncheckedEnum<LineJoin> {
        unsafe { self.ptr.GetLineJoin().into() }
    }

    #[inline]
    pub fn dash_offset(&self) -> f32 {
        unsafe { self.ptr.GetDashOffset() }
    }

    #[inline]
    pub fn dash_style(&self) -> UncheckedEnum<DashStyle> {
        unsafe { self.ptr.GetDashStyle().into() }
    }

    #[inline]
    pub fn dashes_count(&self) -> u32 {
        unsafe { self.ptr.GetDashesCount() }
    }

    #[inline]
    pub fn dashes(&self) -> Vec<f32> {
        let count = self.dashes_count();
        let mut data = vec![0.0; count as usize];
        unsafe {
            self.ptr.GetDashes(data.as_mut_ptr(), count);
        }
        data
    }
}
