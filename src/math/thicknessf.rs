use std::ops::{Div, Mul, Neg};

use winapi::um::d2d1::D2D1_RECT_F;

#[derive(Copy, Clone)]
#[repr(C)]
/// 2D real thickness (left, top, right, bottom).
pub struct ThicknessF(pub D2D1_RECT_F);

math_wrapper!(ThicknessF: D2D1_RECT_F);

impl ThicknessF {
    #[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> ThicknessF {
        ThicknessF(D2D1_RECT_F {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        })
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.right + self.left
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.bottom + self.top
    }
}

impl From<(f32, f32, f32, f32)> for ThicknessF {
    #[inline]
    fn from((left, top, right, bottom): (f32, f32, f32, f32)) -> ThicknessF {
        ThicknessF::new(left, top, right, bottom)
    }
}

impl Neg for ThicknessF {
    type Output = ThicknessF;
    #[inline]
    fn neg(self) -> ThicknessF {
        ThicknessF::new(-self.left, -self.top, -self.right, -self.bottom)
    }
}

impl Mul<f32> for ThicknessF {
    type Output = ThicknessF;
    #[inline]
    fn mul(self, rhs: f32) -> ThicknessF {
        ThicknessF::new(
            self.left * rhs,
            self.top * rhs,
            self.right * rhs,
            self.bottom * rhs,
        )
    }
}

impl Div<f32> for ThicknessF {
    type Output = ThicknessF;
    #[inline]
    fn div(self, rhs: f32) -> ThicknessF {
        ThicknessF::new(
            self.left / rhs,
            self.top / rhs,
            self.right / rhs,
            self.bottom / rhs,
        )
    }
}
