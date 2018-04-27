use image::{Bitmap, Image};

use winapi::um::d2d1::ID2D1Image;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct GenericImage {
    ptr: ComPtr<ID2D1Image>,
}

impl GenericImage {
    #[inline]
    pub fn as_bitmap(&self) -> Option<Bitmap> {
        Some(unsafe { Bitmap::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1Image>) -> Self {
        Self { ptr }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1Image {
        self.ptr.as_raw()
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut ID2D1Image) -> Self {
        GenericImage {
            ptr: ComPtr::from_raw(ptr),
        }
    }
}

impl Image for GenericImage {
    #[inline]
    unsafe fn get_ptr(&self) -> *mut ID2D1Image {
        self.ptr.as_raw()
    }
}

unsafe impl Send for GenericImage {}
unsafe impl Sync for GenericImage {}
