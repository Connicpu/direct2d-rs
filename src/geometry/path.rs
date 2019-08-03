use crate::factory::IFactory;
use crate::geometry::IGeometry;
use crate::resource::IResource;

use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Geometry, ID2D1PathGeometry, ID2D1Resource};
use wio::com::ComPtr;

pub use self::builder::*;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Custom-shaped geometry made of lines and curves
pub struct PathGeometry {
    ptr: ComPtr<ID2D1PathGeometry>,
}

impl PathGeometry {
    pub fn create(factory: &dyn IFactory) -> Result<PathBuilder, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = factory.raw_f().CreatePathGeometry(&mut ptr);
            if SUCCEEDED(hr) {
                let path = PathGeometry::from_raw(ptr);

                let mut ptr = std::ptr::null_mut();
                let hr = path.ptr.Open(&mut ptr);

                if SUCCEEDED(hr) {
                    Ok(PathBuilder {
                        path: path,
                        sink: ComPtr::from_raw(ptr),
                    })
                } else {
                    Err(hr.into())
                }
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn segment_count(&self) -> Result<u32, Error> {
        unsafe {
            let mut count = 0;
            let result = self.ptr.GetSegmentCount(&mut count);
            if SUCCEEDED(result) {
                Ok(count)
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn figure_count(&self) -> Result<u32, Error> {
        unsafe {
            let mut count = 0;
            let result = self.ptr.GetFigureCount(&mut count);
            if SUCCEEDED(result) {
                Ok(count)
            } else {
                Err(From::from(result))
            }
        }
    }
}

unsafe impl IResource for PathGeometry {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IGeometry for PathGeometry {
    unsafe fn raw_geom(&self) -> &ID2D1Geometry {
        &self.ptr
    }
}

unsafe impl super::GeometryType for PathGeometry {}
