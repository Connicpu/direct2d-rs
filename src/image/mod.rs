use helpers::FromRaw;
use math::{SizeF, SizeU};

use winapi::um::d2d1::{ID2D1Bitmap, ID2D1Image};
use wio::com::ComPtr;

pub trait Image {
    unsafe fn get_ptr(&self) -> *mut ID2D1Image;
}

pub struct GenericImage {
    ptr: ComPtr<ID2D1Image>,
}

impl GenericImage {
    pub fn as_bitmap(&self) -> Option<Bitmap> {
        Some(Bitmap {
            ptr: self.ptr.cast().ok()?,
        })
    }
}

impl Image for GenericImage {
    unsafe fn get_ptr(&self) -> *mut ID2D1Image {
        self.ptr.as_raw()
    }
}

unsafe impl Send for GenericImage {}
unsafe impl Sync for GenericImage {}

pub struct Bitmap {
    ptr: ComPtr<ID2D1Bitmap>,
}

impl Bitmap {
    pub fn get_size(&self) -> SizeF {
        unsafe { SizeF(self.ptr.GetSize()) }
    }

    pub fn get_pixel_size(&self) -> SizeU {
        unsafe { SizeU(self.ptr.GetPixelSize()) }
    }

    pub fn get_dpi(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            self.ptr.GetDpi(&mut x, &mut y);
        }
        (x, y)
    }
}

impl Image for Bitmap {
    unsafe fn get_ptr(&self) -> *mut ID2D1Image {
        self.ptr.as_raw() as *mut _
    }
}

impl FromRaw for Bitmap {
    type Raw = ID2D1Bitmap;
    unsafe fn from_raw(ptr: *mut ID2D1Bitmap) -> Self {
        Bitmap {
            ptr: ComPtr::from_raw(ptr),
        }
    }
}

unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}
