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
}

com_wrapper!(Device: ID2D1Device);
