use math::Point2F;

use std::cmp::PartialEq;
use std::f32::EPSILON;
use std::ops::{Add, Div, Mul, Neg};

use winapi::um::d2dbasetypes::*;

#[derive(Copy, Clone)]
#[repr(C)]
/// A vector in 2D real space.
pub struct Vector2F(pub D2D_VECTOR_2F);

math_wrapper!(Vector2F: D2D_VECTOR_2F);

impl Vector2F {
    /// The zero vector, otherwise known as `<0.0, 0.0>`.
    pub const ZERO: Vector2F = Vector2F(D2D_VECTOR_2F { x: 0.0, y: 0.0 });

    #[inline]
    pub fn new(x: f32, y: f32) -> Vector2F {
        Vector2F(D2D_VECTOR_2F { x: x, y: y })
    }

    #[inline]
    pub fn zero() -> Vector2F {
        Vector2F::new(0.0, 0.0)
    }

    #[inline]
    /// Computes the dot product of this vector and another vector.
    /// Geometrically, `dot(a, b) == ||a|| * ||b|| * cos(θ)` where θ is the angle between the
    /// two vectors. If both vectors are unit vectors, you can take the arc cosine to get the
    /// angle between them. You can also compare the dot product with 0 to determine if the
    /// angle between the vectors is greater or less than 90 degrees without having to normalize
    /// the vectors (a relatively expensive operation as it involves a square root).
    pub fn dot(self, other: Vector2F) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    /// Gets the squared length of this vector.
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    /// Gets the length of this vector.
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    #[inline]
    /// Gets a unit vector pointing in the same direction.
    pub fn normalized(self) -> Vector2F {
        self / self.length()
    }

    #[inline]
    /// This is, strictly speaking, not a 'cross product' in the proper sense, but useful for
    /// math nonetheless. The 'cross' of two 2D vectors is `||a|| * ||b|| * sin(θ)`. If both vectors
    /// are unit vectors, you can take the arc sine to get the angle between the two vectors, and
    /// determine which one is on the left or right, information which you can combine with the
    /// similar property from the dot product to get a full picture.
    pub fn cross(self, other: Vector2F) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl From<(f32, f32)> for Vector2F {
    #[inline]
    /// Constructs the vector from (x, y).
    fn from((x, y): (f32, f32)) -> Vector2F {
        Vector2F::new(x, y)
    }
}

impl Add for Vector2F {
    type Output = Vector2F;

    #[inline]
    /// Linearly combines the two vectors.
    fn add(self, rhs: Vector2F) -> Vector2F {
        Vector2F::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Point2F> for Vector2F {
    type Output = Point2F;

    #[inline]
    /// Translates the point by the vector.
    fn add(self, rhs: Point2F) -> Point2F {
        rhs + self
    }
}

impl Neg for Vector2F {
    type Output = Vector2F;

    #[inline]
    /// Changes `(x, y)` into `(-x, -y)`.
    fn neg(self) -> Vector2F {
        Vector2F::new(-self.x, -self.y)
    }
}

impl Mul<f32> for Vector2F {
    type Output = Vector2F;

    #[inline]
    /// Scales the vector by the scalar value.
    fn mul(self, rhs: f32) -> Vector2F {
        Vector2F::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vector2F> for f32 {
    type Output = Vector2F;

    #[inline]
    /// Scales the vector by the scalar value.
    fn mul(self, rhs: Vector2F) -> Vector2F {
        rhs * self
    }
}

impl Div<f32> for Vector2F {
    type Output = Vector2F;

    #[inline]
    /// Divides the vector by the scalar component-wise.
    fn div(self, rhs: f32) -> Vector2F {
        Vector2F::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<Vector2F> for f32 {
    type Output = Vector2F;

    #[inline]
    /// Divides the scalar by the vector component-wise.
    fn div(self, rhs: Vector2F) -> Vector2F {
        Vector2F::new(self / rhs.x, self / rhs.y)
    }
}

impl PartialEq for Vector2F {
    #[inline]
    fn eq(&self, rhs: &Vector2F) -> bool {
        (self.x - rhs.x).abs() < EPSILON && (self.y - rhs.y).abs() < EPSILON
    }
}
