use math::{Point2F, SizeF};

use std::cmp::PartialEq;
use std::f32::EPSILON;

use winapi::um::d2d1::D2D1_RECT_F;

#[derive(Copy, Clone)]
#[repr(C)]
/// 2D real rectangle (left, top, right, bottom).
pub struct RectF(pub D2D1_RECT_F);

math_wrapper!(RectF: D2D1_RECT_F);

impl RectF {
    #[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> RectF {
        RectF(D2D1_RECT_F {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        })
    }

    #[inline]
    /// Constructs a rectangle where each point represents an opposite corner.
    pub fn bounds(p1: Point2F, p2: Point2F) -> RectF {
        RectF::new(
            f32::min(p1.0.x, p2.0.x),
            f32::min(p1.0.y, p2.0.y),
            f32::max(p1.0.x, p2.0.x),
            f32::max(p1.0.y, p2.0.y),
        )
    }

    #[inline]
    /// Constructs a rectangle with the same top-left corner as the old one,
    /// but a new size specified.
    pub fn with_size(old_rect: RectF, size: SizeF) -> RectF {
        RectF::new(
            old_rect.left,
            old_rect.top,
            old_rect.left + size.width,
            old_rect.top + size.height,
        )
    }

    #[inline]
    /// Constructs a rectangle with the same bottom-right corner, and the
    /// top-left corner adjusted by the size specified.
    pub fn adjusted_by(old_rect: RectF, size: SizeF) -> RectF {
        RectF::new(
            old_rect.left + size.width,
            old_rect.top + size.height,
            old_rect.right,
            old_rect.bottom,
        )
    }

    #[inline]
    /// Constructs a rectangle from center and full extents.
    pub fn around(center: Point2F, size: SizeF) -> RectF {
        let half_diag = (size.width / 2.0, size.height / 2.0).into();
        RectF::bounds(center - half_diag, center + half_diag)
    }

    #[inline]
    /// Determines if the rectangle contains a given point.
    pub fn contains(&self, point: Point2F) -> bool {
        return self.left < point.0.x && self.top < point.0.y && self.right > point.0.x
            && self.bottom > point.0.y;
    }

    #[inline]
    /// Gets the width of the rectangle i.e. the space between the right and left.
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    #[inline]
    /// Gets the height of the rectangle i.e. the space between the bottom and top.
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }

    #[inline]
    pub fn top_left(&self) -> Point2F {
        (self.left, self.top).into()
    }

    #[inline]
    pub fn top_right(&self) -> Point2F {
        (self.right, self.top).into()
    }

    #[inline]
    pub fn bottom_left(&self) -> Point2F {
        (self.left, self.bottom).into()
    }

    #[inline]
    pub fn bottom_right(&self) -> Point2F {
        (self.right, self.bottom).into()
    }
}

impl From<(f32, f32, f32, f32)> for RectF {
    #[inline]
    /// Constructs a rect from `(left, top, right, bottom)`.
    fn from((left, top, right, bottom): (f32, f32, f32, f32)) -> RectF {
        RectF::new(left, top, right, bottom)
    }
}

impl PartialEq for RectF {
    #[inline]
    fn eq(&self, rhs: &RectF) -> bool {
        return (self.left - rhs.left).abs() < EPSILON && (self.top - rhs.top).abs() < EPSILON
            && (self.right - rhs.right).abs() < EPSILON
            && (self.bottom - rhs.bottom).abs() < EPSILON;
    }
}
