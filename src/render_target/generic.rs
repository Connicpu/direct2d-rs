use device_context::DeviceContext;
use render_target::{HwndRenderTarget, RenderTarget};

use com_wrapper::ComWrapper;
use winapi::um::d2d1::ID2D1RenderTarget;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper)]
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
}

impl RenderTarget for GenericRenderTarget {
    #[inline]
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *self.ptr.as_raw()
    }
}

unsafe impl Send for GenericRenderTarget {}
unsafe impl Sync for GenericRenderTarget {}
