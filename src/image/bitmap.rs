use crate::descriptions::PixelFormat;
use crate::image::IImage;
use crate::render_target::IRenderTarget;
use crate::resource::IResource;

use com_wrapper::ComWrapper;
use math2d::{Sizef, Sizeu};
use winapi::um::d2d1::{ID2D1Bitmap, ID2D1Image, ID2D1Resource};
use wio::com::ComPtr;

pub use self::builder::*;
pub use self::shared::*;

pub mod builder;
pub mod shared;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Bitmap {
    ptr: ComPtr<ID2D1Bitmap>,
}

impl Bitmap {
    pub fn create<'a>(context: &'a dyn IRenderTarget) -> BitmapBuilder<'a> {
        BitmapBuilder::new(context)
    }
}

pub unsafe trait IBitmap: IImage {
    fn size(&self) -> Sizef {
        unsafe { self.raw_bitmap().GetSize().into() }
    }

    fn pixel_size(&self) -> Sizeu {
        unsafe { self.raw_bitmap().GetPixelSize().into() }
    }

    fn pixel_format(&self) -> PixelFormat {
        unsafe { self.raw_bitmap().GetPixelFormat().into() }
    }

    fn dpi(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            self.raw_bitmap().GetDpi(&mut x, &mut y);
        }
        (x, y)
    }

    unsafe fn raw_bitmap(&self) -> &ID2D1Bitmap;
}

unsafe impl IResource for Bitmap {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IImage for Bitmap {
    unsafe fn raw_img(&self) -> &ID2D1Image {
        &self.ptr
    }
}

unsafe impl IBitmap for Bitmap {
    unsafe fn raw_bitmap(&self) -> &ID2D1Bitmap {
        &self.ptr
    }
}
