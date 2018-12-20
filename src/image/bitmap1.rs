use crate::device_context::DeviceContext;

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

impl std::ops::Deref for Bitmap1 {
    type Target = super::Bitmap;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

