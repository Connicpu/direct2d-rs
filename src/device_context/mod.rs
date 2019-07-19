use crate::device::Device;
use crate::error::D2DResult;
use crate::image::Image;
use crate::render_target::{RTState, RenderTarget};

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1_1::ID2D1DeviceContext;
use wio::com::ComPtr;

#[repr(C)]
pub struct DeviceContext {
    ptr: ComPtr<ID2D1DeviceContext>,
    state: RTState,
}

impl DeviceContext {
    #[inline]
    pub fn create(device: &Device) -> D2DResult<DeviceContext> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*device.get_raw()).CreateDeviceContext(1, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(DeviceContext::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn set_target(&mut self, target: &Image) {
        /*if !self.state.is_set(RTState::NOT_DRAWING) {
            panic!(
                "You should not call `DeviceContext::set_target` while \
                 the target is being drawn to."
            );
        }*/

        unsafe {
            self.ptr.SetTarget(target.get_raw());
            self.state.clear(RTState::NO_TARGET_IMAGE);
        }
    }
}

impl std::ops::Deref for DeviceContext {
    type Target = RenderTarget;
    fn deref(&self) -> &RenderTarget {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for DeviceContext {
    fn deref_mut(&mut self) -> &mut RenderTarget {
        unsafe { dcommon::helpers::deref_com_wrapper_mut(self) }
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
