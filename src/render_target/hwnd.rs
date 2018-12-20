use crate::enums::WindowState;
use crate::error::D2DResult;
use crate::factory::Factory;
use crate::render_target::{RTState, RenderTarget};

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use math2d::Sizeu;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1HwndRenderTarget;
use wio::com::ComPtr;

pub use self::builder::HwndRenderTargetBuilder;

pub mod builder;

#[repr(C)]
pub struct HwndRenderTarget {
    ptr: ComPtr<ID2D1HwndRenderTarget>,
    state: RTState,
}

impl HwndRenderTarget {
    #[inline]
    pub fn create<'a>(factory: &'a Factory) -> HwndRenderTargetBuilder<'a> {
        HwndRenderTargetBuilder::new(factory)
    }

    #[inline]
    pub fn window_state(&self) -> UncheckedEnum<WindowState> {
        unsafe { self.ptr.CheckWindowState().into() }
    }

    #[inline]
    pub fn resize(&self, pixel_size: Sizeu) -> D2DResult<()> {
        unsafe {
            let hr = self.ptr.Resize(&pixel_size.into());
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn hwnd(&self) -> HWND {
        unsafe { self.ptr.GetHwnd() }
    }
}

impl std::ops::Deref for HwndRenderTarget {
    type Target = RenderTarget;
    fn deref(&self) -> &RenderTarget {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for HwndRenderTarget {
    fn deref_mut(&mut self) -> &mut RenderTarget {
        unsafe { dcommon::helpers::deref_com_wrapper_mut(self) }
    }
}

impl super::RenderTargetType for HwndRenderTarget {}

impl ComWrapper for HwndRenderTarget {
    type Interface = ID2D1HwndRenderTarget;
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
        HwndRenderTarget {
            ptr,
            state: RTState::NOT_DRAWING,
        }
    }
    unsafe fn into_ptr(self) -> ComPtr<Self::Interface> {
        self.ptr
    }
}

unsafe impl Send for HwndRenderTarget {}
unsafe impl Sync for HwndRenderTarget {}

impl std::fmt::Debug for HwndRenderTarget {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("HwndRenderTarget")
            .field("ptr", &self.ptr.as_raw())
            .field("state", &self.state)
            .finish()
    }
}

impl std::convert::AsRef<RenderTarget> for HwndRenderTarget {
    fn as_ref(&self) -> &RenderTarget {
        self
    }
}
