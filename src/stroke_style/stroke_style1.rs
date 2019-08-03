use crate::enums::StrokeTransformType;
use crate::factory::IFactory1;
use crate::resource::IResource;
use crate::stroke_style::IStrokeStyle;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use winapi::um::d2d1::{ID2D1Resource, ID2D1StrokeStyle};
use winapi::um::d2d1_1::ID2D1StrokeStyle1;
use wio::com::ComPtr;

pub use self::builder::*;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct StrokeStyle1 {
    ptr: ComPtr<ID2D1StrokeStyle1>,
}

impl StrokeStyle1 {
    #[inline]
    pub fn create<'a>(factory: &'a dyn IFactory1) -> StrokeStyleBuilder1<'a> {
        StrokeStyleBuilder1::new(factory)
    }
}

pub unsafe trait IStrokeStyle1: IStrokeStyle {
    fn transform_type(&self) -> UncheckedEnum<StrokeTransformType> {
        unsafe { self.raw_stroke1().GetStrokeTransformType().into() }
    }

    unsafe fn raw_stroke1(&self) -> &ID2D1StrokeStyle1;
}

unsafe impl IResource for StrokeStyle1 {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IStrokeStyle for StrokeStyle1 {
    unsafe fn raw_stroke(&self) -> &ID2D1StrokeStyle {
        &self.ptr
    }
}

unsafe impl IStrokeStyle1 for StrokeStyle1 {
    unsafe fn raw_stroke1(&self) -> &ID2D1StrokeStyle1 {
        &self.ptr
    }
}
