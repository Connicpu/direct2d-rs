use device::Device;
use error::D2DResult;
use image::Image;
use render_target::RenderTarget;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1RenderTarget;
use winapi::um::d2d1_1::ID2D1DeviceContext;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct DeviceContext {
    ptr: ComPtr<ID2D1DeviceContext>,
}

impl DeviceContext {
    #[inline]
    pub fn create(device: &Device, multithread_context: bool) -> D2DResult<DeviceContext> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*device.get_raw()).CreateDeviceContext(multithread_context as u32, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(DeviceContext::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn set_target<I>(&mut self, target: &I)
    where
        I: Image,
    {
        unsafe {
            self.ptr.SetTarget(target.get_ptr());
        }
    }
}

com_wrapper!(DeviceContext: ID2D1DeviceContext);

impl RenderTarget for DeviceContext {
    #[inline]
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *(self.ptr.as_raw() as *mut _)
    }
}

