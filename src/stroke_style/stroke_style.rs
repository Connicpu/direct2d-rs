use crate::enums::{CapStyle, DashStyle, LineJoin};
use crate::factory::IFactory;
use crate::resource::IResource;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use winapi::um::d2d1::{ID2D1Resource, ID2D1StrokeStyle};
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
    pub fn create<'a>(factory: &'a dyn IFactory) -> StrokeStyleBuilder<'a> {
        StrokeStyleBuilder::new(factory)
    }
}

pub unsafe trait IStrokeStyle: IResource {
    fn start_cap(&self) -> UncheckedEnum<CapStyle> {
        unsafe { self.raw_stroke().GetStartCap().into() }
    }

    fn end_cap(&self) -> UncheckedEnum<CapStyle> {
        unsafe { self.raw_stroke().GetEndCap().into() }
    }

    fn dash_cap(&self) -> UncheckedEnum<CapStyle> {
        unsafe { self.raw_stroke().GetDashCap().into() }
    }

    fn miter_limit(&self) -> f32 {
        unsafe { self.raw_stroke().GetMiterLimit() }
    }

    fn line_join(&self) -> UncheckedEnum<LineJoin> {
        unsafe { self.raw_stroke().GetLineJoin().into() }
    }

    fn dash_offset(&self) -> f32 {
        unsafe { self.raw_stroke().GetDashOffset() }
    }

    fn dash_style(&self) -> UncheckedEnum<DashStyle> {
        unsafe { self.raw_stroke().GetDashStyle().into() }
    }

    fn dashes_count(&self) -> u32 {
        unsafe { self.raw_stroke().GetDashesCount() }
    }

    fn dashes(&self) -> Vec<f32> {
        let count = self.dashes_count();
        let mut data = vec![0.0; count as usize];
        unsafe {
            self.raw_stroke().GetDashes(data.as_mut_ptr(), count);
        }
        data
    }

    unsafe fn raw_stroke(&self) -> &ID2D1StrokeStyle;
}

unsafe impl IResource for StrokeStyle {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IStrokeStyle for StrokeStyle {
    unsafe fn raw_stroke(&self) -> &ID2D1StrokeStyle {
        &self.ptr
    }
}
