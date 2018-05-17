use std::cmp::PartialEq;
use std::f32::EPSILON;

use winapi::um::d2d1::D2D1_SIZE_F;

#[derive(Copy, Clone)]
#[repr(C)]
/// 2D real size (width, height).
pub struct SizeF(pub D2D1_SIZE_F);

math_wrapper!(SizeF: D2D1_SIZE_F);

impl SizeF {
    #[inline]
    /// Constructs a new size from values
    pub fn new(width: f32, height: f32) -> SizeF {
        SizeF(D2D1_SIZE_F {
            width: width,
            height: height,
        })
    }
}

impl From<(f32, f32)> for SizeF {
    #[inline]
    /// Constructs a size from (width, height)
    fn from((x, y): (f32, f32)) -> SizeF {
        SizeF::new(x, y)
    }
}

impl PartialEq for SizeF {
    #[inline]
    fn eq(&self, rhs: &SizeF) -> bool {
        return (self.width - rhs.width).abs() < EPSILON
            && (self.height - rhs.height).abs() < EPSILON;
    }
}
