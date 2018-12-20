use crate::error::D2DResult;
use crate::factory::Factory;

use std::{mem, ptr};

use dcommon::helpers::deref_com_wrapper;
use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_ELLIPSE, ID2D1EllipseGeometry};
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents an ellipse which can be used anywhere Geometry is needed
pub struct EllipseGeometry {
    ptr: ComPtr<ID2D1EllipseGeometry>,
}

impl EllipseGeometry {
    #[inline]
    pub fn create<R>(factory: &Factory, ellipse: &math2d::Ellipse) -> D2DResult<EllipseGeometry> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateEllipseGeometry((&ellipse) as *const _ as *const _, &mut ptr);

            if SUCCEEDED(hr) {
                Ok(EllipseGeometry::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_ellipse(&self) -> math2d::Ellipse {
        unsafe {
            let mut ellipse: D2D1_ELLIPSE = mem::uninitialized();
            self.ptr.GetEllipse(&mut ellipse);
            ellipse.into()
        }
    }
}

impl std::ops::Deref for EllipseGeometry {
    type Target = super::Geometry;
    fn deref(&self) -> &super::Geometry {
        unsafe { deref_com_wrapper(self) }
    }
}

unsafe impl super::GeometryType for EllipseGeometry {}
