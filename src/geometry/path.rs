use crate::error::D2DResult;
use crate::factory::Factory;

use com_wrapper::ComWrapper;
use dcommon::helpers::deref_com_wrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1PathGeometry;
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
    #[inline]
    pub fn create(factory: &Factory) -> D2DResult<PathBuilder> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreatePathGeometry(&mut ptr);
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

    #[inline]
    pub fn segment_count(&self) -> D2DResult<u32> {
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

    #[inline]
    pub fn figure_count(&self) -> D2DResult<u32> {
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

impl std::ops::Deref for PathGeometry {
    type Target = super::Geometry;
    fn deref(&self) -> &super::Geometry {
        unsafe { deref_com_wrapper(self) }
    }
}

unsafe impl super::GeometryType for PathGeometry {}
