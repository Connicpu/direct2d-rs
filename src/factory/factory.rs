use crate::error::D2DResult;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{
    D2D1CreateFactory, ID2D1Factory, D2D1_DEBUG_LEVEL_WARNING, D2D1_FACTORY_OPTIONS,
    D2D1_FACTORY_TYPE_MULTI_THREADED,
};
use winapi::Interface;
use wio::com::ComPtr;

#[derive(ComWrapper, Clone, PartialEq)]
#[com(send, sync, debug)]
pub struct Factory {
    ptr: ComPtr<ID2D1Factory>,
}

impl Factory {
    #[inline]
    pub fn new() -> D2DResult<Factory> {
        unsafe {
            let mut ptr: *mut ID2D1Factory = ptr::null_mut();
            let hr = D2D1CreateFactory(
                D2D1_FACTORY_TYPE_MULTI_THREADED,
                &ID2D1Factory::uuidof(),
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
    pub fn desktop_dpi(&self) -> (f32, f32) {
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            self.ptr.GetDesktopDpi(&mut x, &mut y);
            (x, y)
        }
    }
}
