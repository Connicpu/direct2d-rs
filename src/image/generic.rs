use image::{Bitmap, Image};

use winapi::um::d2d1::{ID2D1Bitmap, ID2D1Image};
use wio::com::ComPtr;

pub struct GenericImage {
    ptr: ComPtr<ID2D1Image>,
}

impl GenericImage {
    pub fn as_bitmap(&self) -> Option<Bitmap> {
        Some(unsafe { Bitmap::from_raw(self.ptr.cast::<ID2D1Bitmap>().ok()?.into_raw()) })
    }

    pub unsafe fn get_raw(&self) -> *mut ID2D1Image {
        self.ptr.as_raw()
    }

    pub unsafe fn from_raw(ptr: *mut ID2D1Image) -> Self {
        GenericImage {
            ptr: ComPtr::from_raw(ptr),
        }
    }
}

impl Image for GenericImage {
    unsafe fn get_ptr(&self) -> *mut ID2D1Image {
        self.ptr.as_raw()
    }
}

unsafe impl Send for GenericImage {}
unsafe impl Sync for GenericImage {}
