use math::{Point2F, Vector2F};

use std::cmp::PartialEq;
use std::f32::{EPSILON, MIN_POSITIVE};
use std::ops::Mul;

use winapi::um::d2d1::{D2D1_MATRIX_3X2_F, D2D1_POINT_2F};
use winapi::um::d2dbasetypes::D2D_VECTOR_2F;

#[derive(Copy, Clone)]
#[repr(C)]
/// 2D Affine Transformation Matrix
///
/// Represents an affine transformations in 2D space of points and vectors.
/// Convention is for Row-major matrices, with an implicit right column of (0, 0, 1).
///
/// ```
/// # let (a, b, c, d, x, y) = (0, 0, 0, 0, 0, 0);
/// # let _ = [
/// [a, b, 0]
/// # ,
/// [c, d, 0]
/// # ,
/// [x, y, 1]
/// # ]
pub struct Matrix3x2F(pub D2D1_MATRIX_3X2_F);

math_wrapper!(Matrix3x2F: D2D1_MATRIX_3X2_F);

impl Matrix3x2F {
    /// The Identity matrix. This is the no-op transformation.
    ///
    /// ```
    /// # let _ = [
    /// [1, 0, 0]
    /// # ,
    /// [0, 1, 0]
    /// # ,
    /// [0, 0, 1]
    /// # ];
    pub const IDENTITY: Matrix3x2F = Matrix3x2F(D2D1_MATRIX_3X2_F {
        matrix: [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]],
    });

    #[inline(always)]
    /// Creates a matrix from row-major data.
    pub fn new(data: [[f32; 2]; 3]) -> Matrix3x2F {
        Matrix3x2F(D2D1_MATRIX_3X2_F { matrix: data })
    }

    #[inline(always)]
    /// Gets the inner data as an array.
    pub fn as_array(&self) -> &[[f32; 2]; 3] {
        &self.0.matrix
    }

    #[inline]
    /// Useful for destructuring. Returns `((a, b), (c, d), (x, y))`.
    pub fn as_tuple(&self) -> ((f32, f32), (f32, f32), (f32, f32)) {
        (
            (self.matrix[0][0], self.matrix[0][1]),
            (self.matrix[1][0], self.matrix[1][1]),
            (self.matrix[2][0], self.matrix[2][1]),
        )
    }

    #[inline]
    /// Returns the [Identity][1] matrix.
    ///
    /// [1]: struct.Matrix3x2F.html#associatedconstant.IDENTITY
    pub fn identity() -> Matrix3x2F {
        Matrix3x2F::IDENTITY
    }

    #[inline]
    /// Constructs a translation matrix.
    ///
    /// ```
    /// # let _ = [
    /// [1.0, 0.0, 0]
    /// # ,
    /// [0.0, 1.0, 0]
    /// # ,
    /// [v.x, v.y, 1]
    /// # ];
    pub fn translation(v: Vector2F) -> Matrix3x2F {
        Matrix3x2F::new([[1.0, 0.0], [0.0, 1.0], [v.x, v.y]])
    }

    #[inline]
    /// Constructs a translation matrix which translates the origin to the point `p`.
    ///
    /// ```
    /// # let _ = [
    /// [  1,   0, 0]
    /// # ,
    /// [  0,   1, 0]
    /// # ,
    /// [p.x, p.y, 1]
    /// # ];
    pub fn translate_to(p: Point2F) -> Matrix3x2F {
        Matrix3x2F::translation(Vector2F::new(p.x, p.y))
    }

    #[inline]
    /// Constructs a scaling matrix which scales with respect to a central point.
    ///
    /// ```
    /// # struct V{x:i32,y:i32}
    /// # let (c, s) = (V{x:0,y:0},V{x:0,y:0});
    /// let x = c.x - s.x * c.x;
    /// let y = c.y - s.y * c.y;
    /// # let _ = [
    /// [s.x,   0, 0]
    /// # ,
    /// [  0, s.y, 0]
    /// # ,
    /// [  x,   y, 1]
    /// # ];
    pub fn scale(scale: Vector2F, center: Point2F) -> Matrix3x2F {
        let trans = Vector2F::new(center.x - scale.x * center.x, center.y - scale.y * center.y);

        Matrix3x2F::new([[scale.x, 0.0], [0.0, scale.y], [trans.x, trans.y]])
    }

    #[inline]
    /// Constructs a rotation matrix that rotates around a central point.
    /// Angle is in radians.
    ///
    /// ```
    /// # cos = 0;
    /// # sin = 0;
    /// let x = x - cos * x - sin * y;
    /// let y = y - cos * y - sin * x;
    /// # let _ = [
    /// [ cos, -sin, 0]
    /// # ,
    /// [ sin,  cos, 0]
    /// # ,
    /// [   x,    y, 1]
    /// # ];
    pub fn rotation(angle: f32, center: Point2F) -> Matrix3x2F {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = center.x;
        let y = center.y;
        let tx = x - cos * x - sin * y;
        let ty = y - cos * y - sin * x;

        Matrix3x2F::new([[cos, -sin], [sin, cos], [tx, ty]])
    }

    #[inline]
    /// Constructs a skew matrix around a central point, a la [CSS skew transforms][1].
    /// Angles are in radians.
    ///
    /// ```
    /// # let ax = 0; let ay = 0;
    /// # struct V{x:i32,y:i32}
    /// # let c = V{x:0,y:0};
    /// # fn tan(_:i32)->i32{0}
    /// let x = -c.y * tan(ay);
    /// let y = -c.x * tan(ax);
    /// # let _ = [
    /// [      1, tan(ax), 0]
    /// # ,
    /// [tan(ay),       1, 0]
    /// # ,
    /// [      x,       y, 1]
    /// # ];
    /// ```
    ///
    /// [1]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/skew
    pub fn skew(angle_x: f32, angle_y: f32, center: Point2F) -> Matrix3x2F {
        let tanx = angle_x.tan();
        let tany = angle_y.tan();
        let x = center.x;
        let y = center.y;

        Matrix3x2F::new([[1.0, tanx], [tany, 1.0], [-y * tany, -x * tanx]])
    }

    #[inline]
    /// Calculates the determinant of the matrix.
    pub fn determinant(&self) -> f32 {
        let ((a, b), (c, d), _) = self.as_tuple();

        a * d - b * c
    }

    #[inline]
    /// Checks if the matrix may be inverted (i.e. the determinant is not 0)
    pub fn is_invertible(&self) -> bool {
        self.determinant().abs() > MIN_POSITIVE
    }

    #[inline]
    /// Inverts the matrix, returns None if the matrix is not invertible.
    ///
    /// ```
    /// # let (a,b,c,d,x,y)=(1,0,0,1,0,0);
    /// let tx = c * y - d * x;
    /// let ty = b * x - a * y;
    /// # let _ = [
    /// [ d / det, -b / det, 0]
    /// # ,
    /// [-c / det,  a / det, 0]
    /// # ,
    /// [tx / det, ty / det, 1]
    /// # ];
    /// ```
    pub fn invert(&self) -> Option<Matrix3x2F> {
        if !self.is_invertible() {
            return None;
        }

        let det = self.determinant();
        let ((a, b), (c, d), (x, y)) = self.as_tuple();
        let tx = c * y - d * x;
        let ty = b * x - a * y;

        Some(Matrix3x2F::new([
            [d / det, -b / det],
            [-c / det, a / det],
            [tx / det, ty / det],
        ]))
    }

    #[inline]
    /// Returns if the given matrix is the identity matrix (or extremely close to it)
    pub fn is_identity(&self) -> bool {
        *self == Matrix3x2F::identity()
    }

    #[inline]
    /// Applies this matrix to a value (vector, point, or matrix). If you're multiplying two
    /// transformation matrices together, think of `self` as "coming after" `other`.
    pub fn transform<T>(self, other: T) -> T
    where
        T: Mul<Matrix3x2F, Output = T>,
    {
        other * self
    }
}

impl Mul for Matrix3x2F {
    type Output = Self;

    #[inline]
    /// Multiplies two matrices together. The one on the left "comes before" the one on the right
    /// since row-major forms of the transformations are used.
    fn mul(self, rhs: Matrix3x2F) -> Matrix3x2F {
        let ((a1, b1), (c1, d1), (x1, y1)) = self.as_tuple();
        let ((a2, b2), (c2, d2), (x2, y2)) = rhs.as_tuple();

        Matrix3x2F::new([
            [a1 * a2 + b1 * c2, a1 * b2 + b1 * d2],
            [a2 * c1 + c2 * d1, b2 * c1 + d1 * d2],
            [x2 + a2 * x1 + c2 * y1, y2 + b2 * x1 + d2 * y1],
        ])
    }
}

impl Mul<Matrix3x2F> for Point2F {
    type Output = Point2F;

    #[inline]
    /// Transforms the point with respect to the matrix.
    fn mul(self, rhs: Matrix3x2F) -> Point2F {
        let ((a, b), (c, d), (x, y)) = rhs.as_tuple();
        let D2D1_POINT_2F { x: px, y: py } = self.0;

        Point2F::new(x + a * px + c * py, y + b * px + d * py)
    }
}

impl Mul<Matrix3x2F> for Vector2F {
    type Output = Vector2F;

    #[inline]
    /// Transforms the vector with respect to the matrix.
    fn mul(self, rhs: Matrix3x2F) -> Vector2F {
        let ((a, b), (c, d), _) = rhs.as_tuple();
        let D2D_VECTOR_2F { x: vx, y: vy } = self.0;

        Vector2F::new(a * vx + c * vy, b * vx + d * vy)
    }
}

impl PartialEq for Matrix3x2F {
    /// Compares matrices by checking that all elements are within f32::EPSILON of eachother
    fn eq(&self, rhs: &Matrix3x2F) -> bool {
        for (r0, r1) in self.0.matrix.iter().zip(rhs.0.matrix.iter()) {
            for (&c0, &c1) in r0.iter().zip(r1.iter()) {
                if (c0 - c1).abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}
