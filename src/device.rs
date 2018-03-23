use helpers::{FromRaw, GetRaw};

use winapi::um::d2d1_1::ID2D1Device;
use wio::com::ComPtr;

pub struct Device {
    ptr: ComPtr<ID2D1Device>,
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
