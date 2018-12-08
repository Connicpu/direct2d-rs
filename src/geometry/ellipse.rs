use error::D2DResult;
use factory::Factory;

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
    #[inline]
    pub fn create<R>(factory: &Factory, ellipse: &math2d::Ellipse) -> D2DResult<Ellipse> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateEllipseGeometry((&ellipse) as *const _ as *const _, &mut ptr);

            if SUCCEEDED(hr) {
                Ok(Ellipse::from_raw(ptr))
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

geometry_type!(Ellipse: ID2D1EllipseGeometry);
