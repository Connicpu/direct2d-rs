use crate::error::D2DResult;
use crate::factory::Factory;
use math2d::RoundedRect;
use std::{mem, ptr};

use com_wrapper::ComWrapper;
use dcommon::helpers::deref_com_wrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1RoundedRectangleGeometry, D2D1_ROUNDED_RECT};
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents a rounded rectangle which can be used anywhere Geometry is needed
pub struct RoundedRectangleGeometry {
    ptr: ComPtr<ID2D1RoundedRectangleGeometry>,
}

impl RoundedRectangleGeometry {
    #[inline]
    pub fn create(
        factory: &Factory,
        rectangle: &RoundedRect,
    ) -> D2DResult<RoundedRectangleGeometry> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw())
                .CreateRoundedRectangleGeometry(rectangle as *const _ as *const _, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(RoundedRectangleGeometry::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_rounded_rect(&self) -> RoundedRect {
        unsafe {
            let mut rect: D2D1_ROUNDED_RECT = mem::uninitialized();
            self.ptr.GetRoundedRect(&mut rect);
            mem::transmute(rect)
        }
    }
}

impl std::ops::Deref for RoundedRectangleGeometry {
    type Target = super::Geometry;
    fn deref(&self) -> &super::Geometry {
        unsafe { deref_com_wrapper(self) }
    }
}

unsafe impl super::GeometryType for RoundedRectangleGeometry {}
