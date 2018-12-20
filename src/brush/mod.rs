use math2d::*;

use winapi::um::d2d1::ID2D1Brush;
use wio::com::ComPtr;

pub use self::bitmap::BitmapBrush;
pub use self::gradient::linear::LinearGradientBrush;
pub use self::gradient::radial::RadialGradientBrush;
pub use self::gradient::GradientStopCollection;
pub use self::solid_color::SolidColorBrush;

pub mod bitmap;
pub mod gradient;
pub mod solid_color;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct Brush {
    ptr: ComPtr<ID2D1Brush>,
}

impl Brush {
    #[inline]
    pub fn opacity(&self) -> f32 {
        unsafe { self.ptr.GetOpacity() }
    }

    #[inline]
    pub fn transform(&self) -> Matrix3x2f {
        unsafe {
            let mut mat: Matrix3x2f = std::mem::uninitialized();
            self.ptr.GetTransform((&mut mat) as *mut _ as *mut _);
            mat
        }
    }
}
