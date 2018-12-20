use crate::error::D2DResult;
use crate::factory::Factory;
use math2d::Rectf;
use std::{mem, ptr};

use com_wrapper::ComWrapper;
use dcommon::helpers::deref_com_wrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1RectangleGeometry, D2D1_RECT_F};
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents a rectangle which can be used anywhere Geometry is needed
pub struct RectangleGeometry {
    ptr: ComPtr<ID2D1RectangleGeometry>,
}

impl RectangleGeometry {
    #[inline]
    pub fn create(factory: &Factory, rectangle: &Rectf) -> D2DResult<RectangleGeometry> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw())
                .CreateRectangleGeometry(rectangle as *const _ as *const _, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(RectangleGeometry::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn rect(&self) -> Rectf {
        unsafe {
            let mut rect: D2D1_RECT_F = mem::uninitialized();
            self.ptr.GetRect(&mut rect);
            mem::transmute(rect)
        }
    }
}

impl std::ops::Deref for RectangleGeometry {
    type Target = super::Geometry;
    fn deref(&self) -> &super::Geometry {
        unsafe { deref_com_wrapper(self) }
    }
}

unsafe impl super::GeometryType for RectangleGeometry {}
