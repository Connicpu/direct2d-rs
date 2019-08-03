use crate::factory::IFactory;
use crate::geometry::IGeometry;
use crate::resource::IResource;

use std::mem::MaybeUninit;

use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1EllipseGeometry, ID2D1Geometry, ID2D1Resource};
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
    pub fn create<R>(
        factory: &dyn IFactory,
        ellipse: &math2d::Ellipse,
    ) -> Result<EllipseGeometry, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = factory
                .raw_f()
                .CreateEllipseGeometry((&ellipse) as *const _ as *const _, &mut ptr);

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
            let mut ellipse = MaybeUninit::uninit();
            self.ptr.GetEllipse(ellipse.as_mut_ptr());
            ellipse.assume_init().into()
        }
    }
}

unsafe impl IResource for EllipseGeometry {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IGeometry for EllipseGeometry {
    unsafe fn raw_geom(&self) -> &ID2D1Geometry {
        &self.ptr
    }
}

unsafe impl super::GeometryType for EllipseGeometry {}
