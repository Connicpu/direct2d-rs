use winapi::*;

pub trait Brush {
    unsafe fn get_ptr(&self) -> *mut ID2D1Brush;
}
