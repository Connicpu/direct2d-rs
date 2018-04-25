use error::D2DResult;
use factory::Factory;
use math;
use std::{mem, ptr};

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_ROUNDED_RECT, ID2D1RoundedRectangleGeometry};
use wio::com::ComPtr;

/// Represents a rounded rectangle which can be used anywhere Geometry is needed
#[repr(C)]
#[derive(Clone)]
pub struct RoundedRectangle {
    ptr: ComPtr<ID2D1RoundedRectangleGeometry>,
}

impl RoundedRectangle {
    #[inline]
    pub fn create(factory: &Factory, rectangle: &math::RoundedRect) -> D2DResult<RoundedRectangle> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateRoundedRectangleGeometry(&rectangle.0, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(RoundedRectangle::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_rounded_rect(&self) -> math::RoundedRect {
        unsafe {
            let mut rect: D2D1_ROUNDED_RECT = mem::uninitialized();
            self.ptr.GetRoundedRect(&mut rect);
            math::RoundedRect(rect)
        }
    }
}

geometry_type!(RoundedRectangle: ID2D1RoundedRectangleGeometry);
