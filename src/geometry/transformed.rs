use geometry::GenericGeometry;
use math;

use std::{mem, ptr};

use winapi::um::d2d1::{D2D1_MATRIX_3X2_F, ID2D1TransformedGeometry};
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
    pub fn get_transform(&self) -> math::Matrix3x2F {
        unsafe {
            let mut matrix: D2D1_MATRIX_3X2_F = mem::uninitialized();
            self.ptr.GetTransform(&mut matrix);
            math::Matrix3x2F(matrix)
        }
    }
}

geometry_type!(Transformed: ID2D1TransformedGeometry);
