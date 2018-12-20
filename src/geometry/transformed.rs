use crate::geometry::Geometry;

use std::{mem, ptr};

use com_wrapper::ComWrapper;
use dcommon::helpers::deref_com_wrapper;
use math2d::Matrix3x2f;
use winapi::um::d2d1::{ID2D1TransformedGeometry, D2D1_MATRIX_3X2_F};
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Another piece of geometry which has had a transformation applied to it
pub struct TransformedGeometry {
    ptr: ComPtr<ID2D1TransformedGeometry>,
}

impl TransformedGeometry {
    #[inline]
    pub fn source_geometry(&self) -> Geometry {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetSourceGeometry(&mut ptr);
            Geometry::from_raw(ptr)
        }
    }

    #[inline]
    pub fn transform(&self) -> Matrix3x2f {
        unsafe {
            let mut matrix: D2D1_MATRIX_3X2_F = mem::uninitialized();
            self.ptr.GetTransform(&mut matrix);
            mem::transmute(matrix)
        }
    }
}

impl std::ops::Deref for TransformedGeometry {
    type Target = super::Geometry;
    fn deref(&self) -> &super::Geometry {
        unsafe { deref_com_wrapper(self) }
    }
}

unsafe impl super::GeometryType for TransformedGeometry {}
