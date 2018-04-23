use device_context::DeviceContext;
use error::Error;
use helpers::{ret_obj, FromRaw, GetRaw};

use std::ptr;

use winapi::um::d2d1_1::ID2D1Device;
use wio::com::ComPtr;

pub struct Device {
    ptr: ComPtr<ID2D1Device>,
}

impl Device {
    pub fn create_device_context(&self) -> Result<DeviceContext, Error> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.CreateDeviceContext(0, &mut ptr);
            ret_obj(hr, ptr)
        }
    }
}

impl FromRaw for Device {
    type Raw = ID2D1Device;
    unsafe fn from_raw(raw: *mut Self::Raw) -> Self {
        Device {
            ptr: ComPtr::from_raw(raw),
        }
    }
}

impl GetRaw for Device {
    type Raw = ID2D1Device;
    unsafe fn get_raw(&self) -> *mut Self::Raw {
        self.ptr.as_raw()
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}
