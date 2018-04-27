use device_context::DeviceContext;
use render_target::{HwndRenderTarget, RenderTarget};

use winapi::um::d2d1::ID2D1RenderTarget;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct GenericRenderTarget {
    ptr: ComPtr<ID2D1RenderTarget>,
}

impl GenericRenderTarget {
    #[inline]
    pub fn as_hwnd(&self) -> Option<HwndRenderTarget> {
        Some(unsafe { HwndRenderTarget::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub fn as_device_context(&self) -> Option<DeviceContext> {
        Some(unsafe { DeviceContext::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1RenderTarget>) -> Self {
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_raw(raw: *mut ID2D1RenderTarget) -> Self {
        GenericRenderTarget {
            ptr: ComPtr::from_raw(raw),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1RenderTarget {
        self.ptr.as_raw()
    }
}

impl RenderTarget for GenericRenderTarget {
    #[inline]
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *self.ptr.as_raw()
    }
}

unsafe impl Send for GenericRenderTarget {}
unsafe impl Sync for GenericRenderTarget {}
