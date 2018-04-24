use winapi::um::d2d1::ID2D1Image;

pub use self::bitmap::Bitmap;
pub use self::generic::GenericImage;

pub mod bitmap;
pub mod generic;

pub trait Image {
    unsafe fn get_ptr(&self) -> *mut ID2D1Image;
}
