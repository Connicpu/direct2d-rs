use render_target::RenderTarget;

use winapi::um::d2d1::ID2D1RenderTarget;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct GenericRenderTarget {
    ptr: ComPtr<ID2D1RenderTarget>,
}

impl GenericRenderTarget {
    pub unsafe fn from_raw(raw: *mut ID2D1RenderTarget) -> Self {
        GenericRenderTarget {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut ID2D1RenderTarget {
        self.ptr.as_raw()
    }
}

impl RenderTarget for GenericRenderTarget {
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *self.ptr.as_raw()
    }
}

unsafe impl Send for GenericRenderTarget {}
unsafe impl Sync for GenericRenderTarget {}
