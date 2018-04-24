use error::D2DResult;
use factory::Factory;
use math;

use std::{mem, ptr};

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_ELLIPSE, ID2D1EllipseGeometry};
use wio::com::ComPtr;

/// Represents an ellipse which can be used anywhere Geometry is needed
#[repr(C)]
#[derive(Clone)]
pub struct Ellipse {
    ptr: ComPtr<ID2D1EllipseGeometry>,
}

impl Ellipse {
    pub fn create<R>(factory: &Factory, ellipse: &math::Ellipse) -> D2DResult<Ellipse> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateEllipseGeometry(&ellipse.0, &mut ptr);

            if SUCCEEDED(hr) {
                Ok(Ellipse::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn get_ellipse(&self) -> math::Ellipse {
        unsafe {
            let mut ellipse: D2D1_ELLIPSE = mem::uninitialized();
            self.ptr.GetEllipse(&mut ellipse);
            math::Ellipse(ellipse)
        }
    }
}

geometry_type!(Ellipse: ID2D1EllipseGeometry);
