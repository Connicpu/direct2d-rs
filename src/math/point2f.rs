use math::Vector2F;

use std::cmp::PartialEq;
use std::f32::EPSILON;
use std::ops::{Add, Mul, Sub};

use winapi::um::d2d1::D2D1_POINT_2F;

#[derive(Copy, Clone)]
#[repr(C)]
/// A point in 2D real space.
pub struct Point2F(pub D2D1_POINT_2F);

math_wrapper!(Point2F: D2D1_POINT_2F);

impl Point2F {
    /// The origin, otherwise known as `(0, 0)`
    pub const ORIGIN: Point2F = Point2F(D2D1_POINT_2F { x: 0.0, y: 0.0 });

    #[inline]
    // Note: can be made const when that becomes stable
    pub fn new(x: f32, y: f32) -> Point2F {
        Point2F(D2D1_POINT_2F { x: x, y: y })
    }

    #[inline]
    // Note: can be made const when that becomes stable
    pub fn origin() -> Point2F {
        Point2F::new(0.0, 0.0)
    }
}

impl From<(f32, f32)> for Point2F {
    #[inline]
    /// Creates the point from (x, y)
    fn from((x, y): (f32, f32)) -> Point2F {
        Point2F::new(x, y)
    }
}

impl Add<Vector2F> for Point2F {
    type Output = Point2F;

    #[inline]
    /// Linearly translates the point by the vector.
    fn add(self, rhs: Vector2F) -> Point2F {
        Point2F::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point2F {
    type Output = Vector2F;

    #[inline]
    /// Gets the vector pointing from the right argument to the left argument.
    fn sub(self, rhs: Point2F) -> Vector2F {
        Vector2F::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Vector2F> for Point2F {
    type Output = Point2F;

    #[inline]
    /// Linearly translates the point by the vector.
    fn sub(self, rhs: Vector2F) -> Point2F {
        Point2F::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Point2F {
    type Output = Point2F;

    #[inline]
    /// Linearly scales the point by the scalar value.
    fn mul(self, rhs: f32) -> Point2F {
        Point2F::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Point2F> for f32 {
    type Output = Point2F;

    #[inline]
    /// Linearly scales the point by the scalar value.
    fn mul(self, rhs: Point2F) -> Point2F {
        Point2F::new(self * rhs.x, self * rhs.y)
    }
}

impl PartialEq for Point2F {
    #[inline]
    fn eq(&self, other: &Point2F) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}
