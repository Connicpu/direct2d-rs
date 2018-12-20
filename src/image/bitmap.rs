use crate::descriptions::PixelFormat;
use crate::render_target::RenderTarget;

use math2d::{Sizef, Sizeu};
use winapi::um::d2d1::ID2D1Bitmap;
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
    #[inline]
    pub fn create<'a>(context: &'a RenderTarget) -> BitmapBuilder<'a> {
        BitmapBuilder::new(context)
    }

    #[inline]
    pub fn size(&self) -> Sizef {
        unsafe { self.ptr.GetSize().into() }
    }

    #[inline]
    pub fn pixel_size(&self) -> Sizeu {
        unsafe { self.ptr.GetPixelSize().into() }
    }

    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { self.ptr.GetPixelFormat().into() }
    }

    #[inline]
    pub fn dpi(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            self.ptr.GetDpi(&mut x, &mut y);
        }
        (x, y)
    }
}

impl std::ops::Deref for Bitmap {
    type Target = super::Image;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}
