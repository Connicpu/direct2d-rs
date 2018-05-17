use std::cmp::PartialEq;
use std::f32::EPSILON;

use winapi::um::d2d1::D2D1_COLOR_F;

#[derive(Copy, Clone)]
#[repr(C)]
/// 4-component float color (R, G, B, A).
pub struct ColorF(pub D2D1_COLOR_F);

math_wrapper!(ColorF: D2D1_COLOR_F);

impl ColorF {
    // Note: can be made const when that becomes stable
    #[inline]
    pub fn uint_rgb(rgb: u32, a: f32) -> ColorF {
        ColorF(D2D1_COLOR_F {
            r: ((rgb >> 16) & 0xFF) as f32 / 255.0,
            g: ((rgb >> 8) & 0xFF) as f32 / 255.0,
            b: (rgb & 0xFF) as f32 / 255.0,
            a: a,
        })
    }
}

impl<'a> From<&'a ColorF> for ColorF {
    #[inline]
    fn from(color: &'a ColorF) -> ColorF {
        *color
    }
}

impl From<u32> for ColorF {
    #[inline]
    fn from(rgb: u32) -> ColorF {
        ColorF::uint_rgb(rgb, 1.0)
    }
}

impl From<(u32, f32)> for ColorF {
    #[inline]
    fn from((rgb, a): (u32, f32)) -> ColorF {
        ColorF::uint_rgb(rgb, a)
    }
}

impl PartialEq for ColorF {
    #[inline]
    fn eq(&self, rhs: &ColorF) -> bool {
        (self.r - rhs.r).abs() < EPSILON && (self.g - rhs.g).abs() < EPSILON
            && (self.b - rhs.b).abs() < EPSILON && (self.a - rhs.a).abs() < EPSILON
    }
}
