use error::D2DResult;
use factory::Factory;
use math::Rectf;
use std::{mem, ptr};

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1RectangleGeometry, D2D1_RECT_F};
use wio::com::ComPtr;

/// Represents a rectangle which can be used anywhere Geometry is needed
#[repr(C)]
#[derive(Clone)]
pub struct Rectangle {
    ptr: ComPtr<ID2D1RectangleGeometry>,
}

impl Rectangle {
    #[inline]
    pub fn create(factory: &Factory, rectangle: &Rectf) -> D2DResult<Rectangle> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw())
                .CreateRectangleGeometry(rectangle as *const _ as *const _, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(Rectangle::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_rect(&self) -> Rectf {
        unsafe {
            let mut rect: D2D1_RECT_F = mem::uninitialized();
            self.ptr.GetRect(&mut rect);
            mem::transmute(rect)
        }
    }
}

geometry_type!(Rectangle: ID2D1RectangleGeometry);
