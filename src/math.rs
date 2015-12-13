use std::cmp::PartialEq;
use std::ops::{Add, Sub, Neg, Mul, Div};
use winapi::*;
use std::f32::EPSILON;

#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct Point2F(pub D2D1_POINT_2F);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct Vector2F(pub D2D_VECTOR_2F);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct SizeF(pub D2D1_SIZE_F);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct RectF(pub D2D1_RECT_F);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct RoundedRect(pub D2D1_ROUNDED_RECT);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct Ellipse(pub D2D1_ELLIPSE);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct Matrix3x2F(pub D2D1_MATRIX_3X2_F);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct BezierSegment(pub D2D1_BEZIER_SEGMENT);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct QuadBezierSegment(pub D2D1_QUADRATIC_BEZIER_SEGMENT);
#[derive(Copy, Clone, Debug)] #[repr(C)]
pub struct ArcSegment(pub D2D1_ARC_SEGMENT);

pub enum SweepDirection {
    CounterClockwise = 0,
    Clockwise = 1,
}

pub enum ArcSize {
    Small = 0,
    Large = 1,
}

impl Point2F {
    #[inline]
    pub fn new(x: f32, y: f32) -> Point2F {
        Point2F(D2D1_POINT_2F {
            x: x,
            y: y,
        })
    }
    
    pub fn origin() -> Point2F {
        Point2F::new(0.0, 0.0)
    }
}

impl Add<Vector2F> for Point2F {
    type Output = Point2F;
    
    #[inline]
    fn add(self, rhs: Vector2F) -> Point2F {
        Point2F::new(self.0.x + rhs.0.x, self.0.y + rhs.0.y)
    }
}

impl Sub for Point2F {
    type Output = Vector2F;
    
    #[inline]
    fn sub(self, rhs: Point2F) -> Vector2F {
        Vector2F::new(self.0.x - rhs.0.x, self.0.y - rhs.0.y)
    }
}

impl Sub<Vector2F> for Point2F {
    type Output = Point2F;
    
    #[inline]
    fn sub(self, rhs: Vector2F) -> Point2F {
        Point2F::new(self.0.x - rhs.0.x, self.0.y - rhs.0.y)
    }
}

impl Vector2F {
    #[inline]
    pub fn new(x: f32, y: f32) -> Vector2F {
        Vector2F(D2D_VECTOR_2F {
            x: x,
            y: y,
        })
    }
    
    #[inline]
    pub fn zero() -> Vector2F {
        Vector2F::new(0.0, 0.0)
    }
}

impl Add for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn add(self, rhs: Vector2F) -> Vector2F {
        Vector2F::new(self.0.x + rhs.0.x, self.0.y + rhs.0.y)
    }
}

impl Add<Point2F> for Vector2F {
    type Output = Point2F;
    
    #[inline]
    fn add(self, rhs: Point2F) -> Point2F {
        rhs + self
    }
}

impl Neg for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn neg(self) -> Vector2F {
        Vector2F::new(-self.0.x, -self.0.y)
    }
}

impl Mul<f32> for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn mul(self, rhs: f32) -> Vector2F {
        Vector2F::new(self.0.x * rhs, self.0.y * rhs)
    }
}

impl Mul<Vector2F> for f32 {
    type Output = Vector2F;
    
    #[inline]
    fn mul(self, rhs: Vector2F) -> Vector2F {
        rhs * self
    }
}

impl Div<f32> for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn div(self, rhs: f32) -> Vector2F {
        Vector2F::new(self.0.x / rhs, self.0.y / rhs)
    }
}

impl Div<Vector2F> for f32 {
    type Output = Vector2F;
    
    #[inline]
    fn div(self, rhs: Vector2F) -> Vector2F {
        rhs / self
    }
}

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
    pub fn bounds(p1: Point2F, p2: Point2F) -> RectF {
        RectF::new(
            f32::min(p1.0.x, p2.0.x),
            f32::min(p1.0.y, p2.0.y),
            f32::max(p1.0.x, p2.0.x),
            f32::max(p1.0.y, p2.0.y),
        )
    }
}

impl PartialEq for RectF {
    #[inline]
    fn eq(&self, rhs: &RectF) -> bool {
        self.0.left == rhs.0.left &&
        self.0.top == rhs.0.top &&
        self.0.right == rhs.0.right &&
        self.0.bottom == rhs.0.bottom
    }
}

impl Matrix3x2F {
    #[inline]
    pub fn new(data: [[f32; 2]; 3]) -> Matrix3x2F {
        Matrix3x2F(D2D1_MATRIX_3X2_F {
            matrix: data,
        })
    }
    
    #[inline]
    pub fn identity() -> Matrix3x2F {
        Matrix3x2F::new([
            [1.0, 0.0],
            [0.0, 1.0],
            [0.0, 0.0],
        ])
    }
    
    #[inline]
    pub fn translation(v: Vector2F) -> Matrix3x2F {
        let v = v.0;
        Matrix3x2F::new([
            [1.0, 0.0],
            [0.0, 1.0],
            [v.x, v.y],
        ])
    }
    
    #[inline]
    pub fn scale(scale: SizeF, center: Point2F) -> Matrix3x2F {
        let scale = scale.0;
        let center = center.0;
        let trans = Vector2F::new(
            center.x - scale.width * center.x,
            center.y - scale.height * center.y,
        ).0;
        
        Matrix3x2F::new([
            [scale.width,          0.0],
            [        0.0, scale.height],
            [    trans.x,      trans.y],
        ])
    }
    
    #[inline]
    pub fn rotation(angle: f32, center: Point2F) -> Matrix3x2F {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = center.0.x;
        let y = center.0.y;
        let tx = x - cos*x - sin*y;
        let ty = y - cos*y - sin*x;
        
        Matrix3x2F::new([
            [cos, -sin],
            [sin,  cos],
            [ tx,   ty],
        ])
    }
    
    #[inline]
    pub fn skew(angle_x: f32, angle_y: f32, center: Point2F) -> Matrix3x2F {
        let tanx = angle_x.tan();
        let tany = angle_y.tan();
        let x = center.0.x;
        let y = center.0.y;
        
        Matrix3x2F::new([
            [    1.0,    tanx],
            [   tany,     1.0],
            [-y*tany, -x*tanx],
        ])
    }
    
    #[inline]
    pub fn determinant(&self) -> f32 {
        let [[a, b], [c, d], _] = self.0.matrix;
        
        a * d - b * c
    }
    
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.determinant().abs() > EPSILON
    }
    
    #[inline]
    pub fn invert(&self) -> Option<Matrix3x2F> {
        if !self.is_invertible() {
            return None;
        }
        
        let det = self.determinant();
        let [[a, b], [c, d], [x, y]] = self.0.matrix;
        
        Some(Matrix3x2F::new([
            [        d /  det,         b / -det],
            [        c / -det,         a /  det],
            [(d*x-c*y) / -det, (b*x-a*y) /  det],
        ]))
    }
    
    #[inline]
    pub fn is_identity(&self) -> bool {
        *self == Matrix3x2F::identity()
    }
}

impl Mul for Matrix3x2F {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Matrix3x2F) -> Matrix3x2F {
        let [[a1, b1], [c1, d1], [x1, y1]] = self.0.matrix;
        let [[a2, b2], [c2, d2], [x2, y2]] = rhs.0.matrix;
        
        Matrix3x2F::new([
            [     a1 * a2 + b1 * c2,      a1 * b2 + b1 * d2],
            [     a2 * c1 + c2 * d1,      b2 * c1 + d1 * d2],
            [x2 + a2 * x1 + c2 * y1, y2 + b2 * x1 + d2 * y1],
        ])
    }
}

impl Mul<Matrix3x2F> for Point2F {
    type Output = Point2F;
    
    #[inline]
    fn mul(self, rhs: Matrix3x2F) -> Point2F {
        let [[a, b], [c, d], [x, y]] = rhs.0.matrix;
        let D2D1_POINT_2F { x: px, y: py } = self.0;
        
        Point2F::new(x + a*px + c*py, y + b*px + d*py)
    }
}

impl Mul<Matrix3x2F> for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn mul(self, rhs: Matrix3x2F) -> Vector2F {
        let [[a, b], [c, d], _] = rhs.0.matrix;
        let D2D_VECTOR_2F { x: px, y: py } = self.0;
        
        Vector2F::new(a*px + c*py, b*px + d*py)
    }
}

impl PartialEq for Matrix3x2F {
    #[inline]
    fn eq(&self, rhs: &Matrix3x2F) -> bool {
        self.0.matrix == rhs.0.matrix
    }
}

impl BezierSegment {
    pub fn new(p1: Point2F, p2: Point2F, p3: Point2F) -> BezierSegment {
        BezierSegment(D2D1_BEZIER_SEGMENT {
            point1: p1.0,
            point2: p2.0,
            point3: p3.0,
        })
    }
}

impl QuadBezierSegment {
    pub fn new(p1: Point2F, p2: Point2F) -> QuadBezierSegment {
        QuadBezierSegment(D2D1_QUADRATIC_BEZIER_SEGMENT {
            point1: p1.0,
            point2: p2.0,
        })
    }
}

impl ArcSegment {
    pub fn new(
        point: Point2F, size: SizeF, angle: f32, sweep_dir: SweepDirection, arc_size: ArcSize
    ) -> ArcSegment {
        ArcSegment(D2D1_ARC_SEGMENT {
            point: point.0,
            size: size.0,
            rotationAngle: angle,
            sweepDirection: D2D1_SWEEP_DIRECTION(sweep_dir as u32),
            arcSize: D2D1_ARC_SIZE(arc_size as u32),
        })
    }
    
    /// Create a counter-clockwise small arc
    pub fn new_cc_sm(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::CounterClockwise, ArcSize::Small)
    }
    
    /// Create a counter-clockwise large arc
    pub fn new_cc_lg(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::CounterClockwise, ArcSize::Large)
    }
    
    
    /// Create a clockwise small arc
    pub fn new_cw_sm(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::Clockwise, ArcSize::Small)
    }
    
    
    /// Create a counter-clockwise small arc
    pub fn new_cw_lg(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::Clockwise, ArcSize::Large)
    }
}
