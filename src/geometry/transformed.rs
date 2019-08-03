use crate::geometry::{Geometry, IGeometry};
use crate::resource::IResource;

use std::{mem, ptr};

use com_wrapper::ComWrapper;
use math2d::Matrix3x2f;
use winapi::um::d2d1::{ID2D1Geometry, ID2D1Resource, ID2D1TransformedGeometry, D2D1_MATRIX_3X2_F};
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Another piece of geometry which has had a transformation applied to it
pub struct TransformedGeometry {
    ptr: ComPtr<ID2D1TransformedGeometry>,
}

impl TransformedGeometry {
    pub fn source_geometry(&self) -> Geometry {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetSourceGeometry(&mut ptr);
            Geometry::from_raw(ptr)
        }
    }

    pub fn transform(&self) -> Matrix3x2f {
        unsafe {
            let mut matrix: D2D1_MATRIX_3X2_F = mem::uninitialized();
            self.ptr.GetTransform(&mut matrix);
            mem::transmute(matrix)
        }
    }
}

unsafe impl IResource for TransformedGeometry {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IGeometry for TransformedGeometry {
    unsafe fn raw_geom(&self) -> &ID2D1Geometry {
        &self.ptr
    }
}

unsafe impl super::GeometryType for TransformedGeometry {}
