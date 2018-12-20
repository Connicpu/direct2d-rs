use crate::error::D2DResult;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{
    D2D1CreateFactory, D2D1_DEBUG_LEVEL_WARNING, D2D1_FACTORY_OPTIONS,
    D2D1_FACTORY_TYPE_MULTI_THREADED,
};
use winapi::um::d2d1_1::ID2D1Factory1;
use winapi::Interface;
use wio::com::ComPtr;

#[derive(ComWrapper, Clone, PartialEq)]
#[com(send, sync, debug)]
pub struct Factory1 {
    ptr: ComPtr<ID2D1Factory1>,
}

impl Factory1 {
    #[inline]
    pub fn new() -> D2DResult<Factory1> {
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
                Ok(Factory1::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }
}

impl std::ops::Deref for Factory1 {
    type Target = super::Factory;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}
