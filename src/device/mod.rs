use error::D2DResult;
use factory::Factory;

use std::ptr;

use dxgi::device::Device as DxgiDevice;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1_1::ID2D1Device;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct Device {
    ptr: ComPtr<ID2D1Device>,
}

impl Device {
    #[inline]
    pub fn create(factory: &Factory, dxgi: &DxgiDevice) -> D2DResult<Device> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateDevice(dxgi.get_raw(), &mut ptr);
            if SUCCEEDED(hr) {
                Ok(Device::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1Device>) -> Self {
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_raw(raw: *mut ID2D1Device) -> Self {
        Device {
            ptr: ComPtr::from_raw(raw),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1Device {
        self.ptr.as_raw()
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}
