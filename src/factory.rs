use crate::error::D2DResult;

use std::ptr;

use winapi::ctypes::c_void;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1CreateFactory, D2D1_DEBUG_LEVEL_WARNING, D2D1_FACTORY_OPTIONS,
                       D2D1_FACTORY_TYPE_MULTI_THREADED};
use winapi::um::d2d1_1::ID2D1Factory1;
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, PartialEq)]
pub struct Factory {
    ptr: ComPtr<ID2D1Factory1>,
}

impl Factory {
    #[inline]
    pub fn new() -> D2DResult<Factory> {
        unsafe {
            let mut ptr: *mut ID2D1Factory1 = ptr::null_mut();
            let hr = D2D1CreateFactory(
                D2D1_FACTORY_TYPE_MULTI_THREADED,
                &ID2D1Factory1::uuidof(),
                &D2D1_FACTORY_OPTIONS {
                    debugLevel: D2D1_DEBUG_LEVEL_WARNING,
                },
                &mut ptr as *mut _ as *mut *mut c_void,
            );

            if SUCCEEDED(hr) {
                Ok(Factory::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_desktop_dpi(&self) -> (f32, f32) {
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            self.ptr.GetDesktopDpi(&mut x, &mut y);
            (x, y)
        }
    }

    #[inline]
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1Factory1>) -> Self {
        Self { ptr }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1Factory1 {
        self.ptr.as_raw()
    }

    #[inline]
    pub unsafe fn from_raw(raw: *mut ID2D1Factory1) -> Self {
        Factory {
            ptr: ComPtr::from_raw(raw),
        }
    }
}

unsafe impl Send for Factory {}
unsafe impl Sync for Factory {}
