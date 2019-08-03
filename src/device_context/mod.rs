use crate::device::IDevice;
use crate::image::IImage;
use crate::render_target::{IRenderTarget, RTState};
use crate::resource::IResource;

use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1RenderTarget, ID2D1Resource};
use winapi::um::d2d1_1::{
    ID2D1DeviceContext, D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS,
};
use wio::com::ComPtr;

#[repr(C)]
pub struct DeviceContext {
    ptr: ComPtr<ID2D1DeviceContext>,
    state: RTState,
}

impl DeviceContext {
    #[inline]
    pub fn create(device: &dyn IDevice) -> Result<DeviceContext, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = device.raw_dev().CreateDeviceContext(
                D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS,
                &mut ptr,
            );
            if SUCCEEDED(hr) {
                Ok(DeviceContext::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }
}

pub unsafe trait IDeviceContext: IRenderTarget {
    fn set_target(&mut self, target: &dyn IImage) {
        unsafe {
            self.raw_dc().SetTarget(target.raw_img());
            self.draw_state_mut().clear(RTState::NO_TARGET_IMAGE);
        }
    }

    unsafe fn raw_dc(&self) -> &ID2D1DeviceContext;
}

unsafe impl IResource for DeviceContext {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IRenderTarget for DeviceContext {
    unsafe fn raw_rt(&self) -> &ID2D1RenderTarget {
        &self.ptr
    }

    fn draw_state(&self) -> RTState {
        self.state
    }

    fn draw_state_mut(&mut self) -> &mut RTState {
        &mut self.state
    }
}

unsafe impl IDeviceContext for DeviceContext {
    unsafe fn raw_dc(&self) -> &ID2D1DeviceContext {
        &self.ptr
    }
}

impl ComWrapper for DeviceContext {
    type Interface = ID2D1DeviceContext;
    unsafe fn get_raw(&self) -> *mut Self::Interface {
        self.ptr.as_raw()
    }
    unsafe fn into_raw(self) -> *mut Self::Interface {
        self.ptr.into_raw()
    }
    unsafe fn from_raw(raw: *mut Self::Interface) -> Self {
        Self::from_ptr(ComPtr::from_raw(raw))
    }
    unsafe fn from_ptr(ptr: ComPtr<Self::Interface>) -> Self {
        DeviceContext {
            ptr,
            state: RTState::NOT_DRAWING | RTState::NO_TARGET_IMAGE,
        }
    }
    unsafe fn into_ptr(self) -> ComPtr<Self::Interface> {
        self.ptr
    }
}

unsafe impl Send for DeviceContext {}
unsafe impl Sync for DeviceContext {}

impl std::fmt::Debug for DeviceContext {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceContext")
            .field("ptr", &self.ptr.as_raw())
            .field("state", &self.state)
            .finish()
    }
}
