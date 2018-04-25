use error::D2DResult;
use factory::Factory;
use math;
use std::{mem, ptr};

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_RECT_F, ID2D1RectangleGeometry};
use wio::com::ComPtr;

/// Represents a rectangle which can be used anywhere Geometry is needed
#[repr(C)]
#[derive(Clone)]
pub struct Rectangle {
    ptr: ComPtr<ID2D1RectangleGeometry>,
}

impl Rectangle {
    #[inline]
    pub fn create(factory: &Factory, rectangle: &math::RectF) -> D2DResult<Rectangle> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateRectangleGeometry(&rectangle.0, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(Rectangle::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_rect(&self) -> math::RectF {
        unsafe {
            let mut rect: D2D1_RECT_F = mem::uninitialized();
            self.ptr.GetRect(&mut rect);
            math::RectF(rect)
        }
    }
}

geometry_type!(Rectangle: ID2D1RectangleGeometry);
