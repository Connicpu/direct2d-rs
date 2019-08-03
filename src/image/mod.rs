use crate::resource::IResource;

use com_wrapper::ComWrapper;
use winapi::um::d2d1::{ID2D1Image, ID2D1Resource};
use wio::com::ComPtr;

pub use self::bitmap::{Bitmap, IBitmap};
pub use self::bitmap1::{Bitmap1, IBitmap1};

pub mod bitmap;
pub mod bitmap1;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Image {
    ptr: ComPtr<ID2D1Image>,
}

pub unsafe trait IImage: IResource {
    unsafe fn raw_img(&self) -> &ID2D1Image;
}

unsafe impl IResource for Image {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IImage for Image {
    unsafe fn raw_img(&self) -> &ID2D1Image {
        &self.ptr
    }
}
