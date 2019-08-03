use crate::device_context::DeviceContext;
use crate::enums::BitmapOptions;
use crate::image::{IBitmap, IImage};
use crate::resource::IResource;

use com_wrapper::ComWrapper;
use dcommon::Error;
use dxgi::surface::Surface;
use winapi::um::d2d1::{ID2D1Bitmap, ID2D1Image, ID2D1Resource};
use winapi::um::d2d1_1::ID2D1Bitmap1;
use wio::com::ComPtr;

pub use self::builder::*;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Bitmap1 {
    ptr: ComPtr<ID2D1Bitmap1>,
}

impl Bitmap1 {
    pub fn create(ctx: &DeviceContext) -> BitmapBuilder1 {
        BitmapBuilder1::new(ctx)
    }
}

pub unsafe trait IBitmap1: IBitmap {
    /// Gets the options used in creating the bitmap.
    fn options(&self) -> BitmapOptions {
        unsafe { BitmapOptions(self.raw_bitmap1().GetOptions()) }
    }

    fn surface(&self) -> Result<Surface, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_bitmap1().GetSurface(&mut ptr);
            Error::map_if(hr, || Surface::from_raw(ptr))
        }
    }

    unsafe fn raw_bitmap1(&self) -> &ID2D1Bitmap1;
}

unsafe impl IResource for Bitmap1 {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IImage for Bitmap1 {
    unsafe fn raw_img(&self) -> &ID2D1Image {
        &self.ptr
    }
}

unsafe impl IBitmap for Bitmap1 {
    unsafe fn raw_bitmap(&self) -> &ID2D1Bitmap {
        &self.ptr
    }
}

unsafe impl IBitmap1 for Bitmap1 {
    unsafe fn raw_bitmap1(&self) -> &ID2D1Bitmap1 {
        &self.ptr
    }
}
