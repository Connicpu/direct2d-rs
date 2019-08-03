use crate::factory::IFactory;
use crate::geometry::IGeometry;
use crate::resource::IResource;

use std::mem::MaybeUninit;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Rectf;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Geometry, ID2D1RectangleGeometry, ID2D1Resource};
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents a rectangle which can be used anywhere Geometry is needed
pub struct RectangleGeometry {
    ptr: ComPtr<ID2D1RectangleGeometry>,
}

impl RectangleGeometry {
    pub fn create(factory: &dyn IFactory, rectangle: &Rectf) -> Result<RectangleGeometry, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = factory
                .raw_f()
                .CreateRectangleGeometry(rectangle as *const _ as *const _, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(RectangleGeometry::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn rect(&self) -> Rectf {
        unsafe {
            let mut rect = MaybeUninit::uninit();
            self.ptr.GetRect(rect.as_mut_ptr());
            rect.assume_init().into()
        }
    }
}

unsafe impl IResource for RectangleGeometry {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IGeometry for RectangleGeometry {
    unsafe fn raw_geom(&self) -> &ID2D1Geometry {
        &self.ptr
    }
}

unsafe impl super::GeometryType for RectangleGeometry {}
