use crate::device::Device;
use crate::error::D2DResult;
use crate::image::Image;
use crate::render_target::RenderTargetType;
use crate::render_target::{RTState, RenderTarget};

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1_1::ID2D1DeviceContext;
use winapi::um::unknwnbase::IUnknown;
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
    pub fn set_target<I>(&mut self, target: &I)
    where
        I: Image,
    {
        /*if !self.state.is_set(RTState::NOT_DRAWING) {
            panic!(
                "You should not call `DeviceContext::set_target` while \
                 the target is being drawn to."
            );
        }*/

        unsafe {
            self.ptr.SetTarget(target.get_ptr());
            self.state.clear(RTState::NO_TARGET_IMAGE);
        }
    }
}

impl std::ops::Deref for DeviceContext {
    type Target = RenderTarget;
    fn deref(&self) -> &RenderTarget {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for DeviceContext {
    fn deref_mut(&mut self) -> &mut RenderTarget {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

pub trait DeviceContextType: RenderTargetType {
    /// Try to cast this factory to a different factory type
    fn try_cast<R: DeviceContextType>(self) -> Option<R>
    where
        Self: Sized,
    {
        unsafe {
            let ptr = self.into_ptr();
            Some(ComWrapper::from_ptr(ptr.cast().ok()?))
        }
    }

    /// Try to temporarily upcast this type to perform an operation
    fn try_with_cast<R: DeviceContextType, V>(&self, f: impl FnOnce(&R) -> V) -> Option<V> {
        unsafe {
            let ptr = self.get_raw() as *mut IUnknown;
            (*ptr).AddRef();
            let ptr = ComPtr::from_raw(ptr);
            let obj = R::from_ptr(ptr.cast().ok()?);
            Some(f(&obj))
        }
    }
}

impl RenderTargetType for DeviceContext {}
impl DeviceContextType for DeviceContext {}

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
