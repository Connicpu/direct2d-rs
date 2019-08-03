use crate::descriptions::GradientStop;

use com_wrapper::ComWrapper;
use winapi::um::d2d1::{ID2D1GradientStopCollection, ID2D1Resource};
use wio::com::ComPtr;

pub use self::builder::GradientStopBuilder;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents an collection of GradientStop objects for linear and radial gradient brushes.
pub struct GradientStopCollection {
    ptr: ComPtr<ID2D1GradientStopCollection>,
}

impl GradientStopCollection {
    /// Get the number of stops in the collection
    pub fn len(&self) -> u32 {
        unsafe { self.ptr.GetGradientStopCount() }
    }

    /// Get all of the stop points
    pub fn stops(&self) -> Vec<GradientStop> {
        unsafe {
            let len = self.len();
            let mut stops: Vec<GradientStop> = Vec::with_capacity(len as usize);
            self.ptr.GetGradientStops(stops.as_mut_ptr() as *mut _, len);
            stops
        }
    }
}

unsafe impl crate::resource::IResource for GradientStopCollection {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}
