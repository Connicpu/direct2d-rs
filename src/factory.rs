use error::D2DResult;

use std::ptr;

use winapi::ctypes::c_void;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{
    D2D1CreateFactory, D2D1_DEBUG_LEVEL_WARNING, D2D1_FACTORY_OPTIONS,
    D2D1_FACTORY_TYPE_MULTI_THREADED,
};
use winapi::um::d2d1_1::{ID2D1Factory1, ID2D1Multithread};
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
    pub fn get_lock(&self) -> Option<Lock> {
        unsafe { Some(Lock::from_ptr(self.ptr.cast().ok()?)) }
    }
}

com_wrapper!(Factory: ID2D1Factory1);

/// Used for locking out Direct2D whenever you're doing non-d2d work on a shared Direct3D/DXGI
/// object to ensure you don't create a race condition.
#[derive(Clone)]
pub struct Lock {
    ptr: ComPtr<ID2D1Multithread>,
}

impl Lock {
    #[inline]
    pub fn lock(&self) -> LockGuard {
        unsafe { self.ptr.Enter() };
        LockGuard { lock: self }
    }

    #[inline]
    pub fn is_protected(&self) -> bool {
        unsafe { self.ptr.GetMultithreadProtected() != 0 }
    }
}

com_wrapper!(Lock: ID2D1Multithread);

pub struct LockGuard<'a> {
    lock: &'a Lock,
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        unsafe { self.lock.ptr.Leave() };
    }
}
