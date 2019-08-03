use crate::factory::IFactory;
use crate::geometry::IGeometry;
use crate::resource::IResource;

use std::mem::MaybeUninit;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::RoundedRect;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Geometry, ID2D1Resource, ID2D1RoundedRectangleGeometry};
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents a rounded rectangle which can be used anywhere Geometry is needed
pub struct RoundedRectangleGeometry {
    ptr: ComPtr<ID2D1RoundedRectangleGeometry>,
}

impl RoundedRectangleGeometry {
    pub fn create(
        factory: &dyn IFactory,
        rectangle: &RoundedRect,
    ) -> Result<RoundedRectangleGeometry, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = factory
                .raw_f()
                .CreateRoundedRectangleGeometry(rectangle as *const _ as *const _, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(RoundedRectangleGeometry::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn get_rounded_rect(&self) -> RoundedRect {
        unsafe {
            let mut rect = MaybeUninit::uninit();
            self.ptr.GetRoundedRect(rect.as_mut_ptr());
            rect.assume_init().into()
        }
    }
}

unsafe impl IResource for RoundedRectangleGeometry {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IGeometry for RoundedRectangleGeometry {
    unsafe fn raw_geom(&self) -> &ID2D1Geometry {
        &self.ptr
    }
}

unsafe impl super::GeometryType for RoundedRectangleGeometry {}
