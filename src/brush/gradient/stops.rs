use crate::descriptions::GradientStop;

use winapi::um::d2d1::ID2D1GradientStopCollection;
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
    #[inline]
    /// Get the number of stops in the collection
    pub fn len(&self) -> u32 {
        unsafe { self.ptr.GetGradientStopCount() }
    }

    #[inline]
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

impl std::ops::Deref for GradientStopCollection {
    type Target = crate::resource::Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

