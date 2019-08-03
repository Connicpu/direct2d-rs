use crate::enums::WindowState;
use crate::factory::IFactory;
use crate::render_target::{IRenderTarget, RTState};
use crate::resource::IResource;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Sizeu;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1HwndRenderTarget, ID2D1RenderTarget, ID2D1Resource};
use wio::com::ComPtr;

pub use self::builder::HwndRenderTargetBuilder;

pub mod builder;

#[repr(C)]
pub struct HwndRenderTarget {
    ptr: ComPtr<ID2D1HwndRenderTarget>,
    state: RTState,
}

impl HwndRenderTarget {
    pub fn create<'a>(factory: &'a dyn IFactory) -> HwndRenderTargetBuilder<'a> {
        HwndRenderTargetBuilder::new(factory)
    }

    pub fn window_state(&self) -> UncheckedEnum<WindowState> {
        unsafe { self.ptr.CheckWindowState().into() }
    }

    pub fn resize(&self, pixel_size: Sizeu) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.Resize(&pixel_size.into());
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn hwnd(&self) -> HWND {
        unsafe { self.ptr.GetHwnd() }
    }
}

unsafe impl IResource for HwndRenderTarget {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IRenderTarget for HwndRenderTarget {
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
