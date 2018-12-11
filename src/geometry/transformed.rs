use crate::geometry::GenericGeometry;
use math2d::Matrix3x2f;

use std::{mem, ptr};

use winapi::um::d2d1::{ID2D1TransformedGeometry, D2D1_MATRIX_3X2_F};
use wio::com::ComPtr;

/// Another piece of geometry which has had a transformation applied to it
#[repr(C)]
#[derive(Clone)]
pub struct Transformed {
    ptr: ComPtr<ID2D1TransformedGeometry>,
}

impl Transformed {
    #[inline]
    pub fn get_source_geometry(&self) -> GenericGeometry {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetSourceGeometry(&mut ptr);
            GenericGeometry::from_raw(ptr)
        }
    }

    #[inline]
    pub fn get_transform(&self) -> Matrix3x2f {
        unsafe {
            let mut matrix: D2D1_MATRIX_3X2_F = mem::uninitialized();
            self.ptr.GetTransform(&mut matrix);
            mem::transmute(matrix)
        }
    }
}

geometry_type!(Transformed: ID2D1TransformedGeometry);
