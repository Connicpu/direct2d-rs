use crate::factory::IFactory1;
use crate::resource::IResource;

use com_wrapper::ComWrapper;
use dcommon::Error;
use dxgi::device::IDevice as IDxgiDevice;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1Resource;
use winapi::um::d2d1_1::ID2D1Device;
use wio::com::ComPtr;

#[derive(ComWrapper, PartialEq)]
#[com(send, sync, debug)]
pub struct Device {
    ptr: ComPtr<ID2D1Device>,
}

impl Device {
    #[inline]
    pub fn create(factory: &dyn IFactory1, dxgi: &dyn IDxgiDevice) -> Result<Device, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = factory.raw_f1().CreateDevice(dxgi.raw_dev(), &mut ptr);
            if SUCCEEDED(hr) {
                Ok(Device::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }
}

pub unsafe trait IDevice: IResource {
    unsafe fn raw_dev(&self) -> &ID2D1Device;
}

unsafe impl IResource for Device {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IDevice for Device {
    unsafe fn raw_dev(&self) -> &ID2D1Device {
        &self.ptr
    }
}
