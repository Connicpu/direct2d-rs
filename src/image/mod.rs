use winapi::um::d2d1::ID2D1Image;
use wio::com::ComPtr;

pub use self::bitmap::Bitmap;
pub use self::bitmap1::Bitmap1;

pub mod bitmap;
pub mod bitmap1;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Image {
    ptr: ComPtr<ID2D1Image>,
}

impl std::ops::Deref for Image {
    type Target = crate::resource::Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}
