use std::cmp::PartialEq;
use std::ops::{Add, Sub, Neg, Mul, Div};
use winapi::*;
use std::f32::EPSILON;

math_wrappers! {
    pub struct Point2F(pub D2D1_POINT_2F);
    pub struct Vector2F(pub D2D_VECTOR_2F);
    pub struct SizeF(pub D2D1_SIZE_F);
    pub struct RectF(pub D2D1_RECT_F);
    pub struct ThicknessF(pub D2D1_RECT_F);
    pub struct RoundedRect(pub D2D1_ROUNDED_RECT);
    pub struct Ellipse(pub D2D1_ELLIPSE);
    pub struct ColorF(pub D2D1_COLOR_F);
    pub struct Matrix3x2F(pub D2D1_MATRIX_3X2_F);
    pub struct BezierSegment(pub D2D1_BEZIER_SEGMENT);
    pub struct QuadBezierSegment(pub D2D1_QUADRATIC_BEZIER_SEGMENT);
    pub struct ArcSegment(pub D2D1_ARC_SEGMENT);
    pub struct BrushProperties(pub D2D1_BRUSH_PROPERTIES);
}

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
    pub const fn new(x: f32, y: f32) -> Point2F {
        Point2F(D2D1_POINT_2F {
            x: x,
            y: y,
        })
    }
    
    #[inline]
    pub const fn origin() -> Point2F {
        Point2F::new(0.0, 0.0)
    }
}

impl From<(f32, f32)> for Point2F {
    #[inline]
    fn from((x, y): (f32, f32)) -> Point2F {
        Point2F::new(x, y)
    }
}

impl Add<Vector2F> for Point2F {
    type Output = Point2F;
    
    #[inline]
    fn add(self, rhs: Vector2F) -> Point2F {
        Point2F::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point2F {
    type Output = Vector2F;
    
    #[inline]
    fn sub(self, rhs: Point2F) -> Vector2F {
        Vector2F::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Vector2F> for Point2F {
    type Output = Point2F;
    
    #[inline]
    fn sub(self, rhs: Vector2F) -> Point2F {
        Point2F::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Point2F {
    type Output = Point2F;
    
    #[inline]
    fn mul(self, rhs: f32) -> Point2F {
        Point2F::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Point2F> for f32 {
    type Output = Point2F;
    
    #[inline]
    fn mul(self, rhs: Point2F) -> Point2F {
        Point2F::new(self * rhs.x, self * rhs.y)
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

impl From<(f32, f32)> for Vector2F {
    #[inline]
    fn from((x, y): (f32, f32)) -> Vector2F {
        Vector2F::new(x, y)
    }
}

impl Add for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn add(self, rhs: Vector2F) -> Vector2F {
        Vector2F::new(self.x + rhs.x, self.y + rhs.y)
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
        Vector2F::new(-self.x, -self.y)
    }
}

impl Mul<f32> for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn mul(self, rhs: f32) -> Vector2F {
        Vector2F::new(self.x * rhs, self.y * rhs)
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
        Vector2F::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<Vector2F> for f32 {
    type Output = Vector2F;
    
    #[inline]
    fn div(self, rhs: Vector2F) -> Vector2F {
        rhs / self
    }
}

impl SizeF {
    #[inline]
    pub fn new(width: f32, height: f32) -> SizeF {
        SizeF(D2D1_SIZE_F {
            width: width,
            height: height,
        })
    }
}

impl From<(f32, f32)> for SizeF {
    #[inline]
    fn from((x, y): (f32, f32)) -> SizeF {
        SizeF::new(x, y)
    }
}

impl PartialEq for SizeF {
    #[inline]
    fn eq(&self, rhs: &SizeF) -> bool {
        return
            self.width == rhs.width &&
            self.height == rhs.height;
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
    
    #[inline]
    pub fn with_size(old_rect: RectF, size: SizeF) -> RectF {
        RectF::new(
            old_rect.left,
            old_rect.top,
            old_rect.left + size.width,
            old_rect.top + size.height,
        )
    }
    
    #[inline]
    pub fn adjusted_by(old_rect: RectF, size: SizeF) -> RectF {
        RectF::new(
            old_rect.left + size.width,
            old_rect.top + size.height,
            old_rect.right,
            old_rect.bottom,
        )
    }
    
    #[inline]
    pub fn around(center: Point2F, size: SizeF) -> RectF {
        let half_diag = (size.width / 2.0, size.height / 2.0).into();
        RectF::bounds(center - half_diag, center + half_diag)
    }
    
    #[inline]
    pub fn contains(&self, point: Point2F) -> bool {
        return
            self.left < point.0.x &&
            self.top < point.0.y &&
            self.right > point.0.x &&
            self.bottom > point.0.y;
    }
    
    #[inline]
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    
    #[inline]
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }
    
    #[inline]
    pub fn top_left(&self) -> Point2F {
        (self.left, self.top).into()
    }
}

impl From<(f32, f32, f32, f32)> for RectF {
    #[inline]
    fn from((left, top, right, bottom): (f32, f32, f32, f32)) -> RectF {
        RectF::new(left, top, right, bottom)
    }
}

impl PartialEq for RectF {
    #[inline]
    fn eq(&self, rhs: &RectF) -> bool {
        self.left == rhs.left &&
        self.top == rhs.top &&
        self.right == rhs.right &&
        self.bottom == rhs.bottom
    }
}

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
    fn neg(self) -> ThicknessF {
        ThicknessF::new(-self.left, -self.top, -self.right, -self.bottom)
    }
}

impl Mul<f32> for ThicknessF {
    type Output = ThicknessF;
    fn mul(self, rhs: f32) -> ThicknessF {
        ThicknessF::new(self.left * rhs, self.top * rhs, self.right * rhs, self.bottom * rhs)
    }
}

impl Div<f32> for ThicknessF {
    type Output = ThicknessF;
    fn div(self, rhs: f32) -> ThicknessF {
        ThicknessF::new(self.left / rhs, self.top / rhs, self.right / rhs, self.bottom / rhs)
    }
}

impl Ellipse {
    #[inline]
    pub fn new(center: Point2F, radius_x: f32, radius_y: f32) -> Ellipse {
        Ellipse(D2D1_ELLIPSE {
            point: center.0,
            radiusX: radius_x,
            radiusY: radius_y,
        })
    }
}

impl ColorF {
    #[inline]
    pub const fn uint_rgb(rgb: u32, a: f32) -> ColorF {
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
        self.r == rhs.r &&
        self.g == rhs.g &&
        self.b == rhs.b &&
        self.a == rhs.a
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
        Matrix3x2F::new([
            [1.0, 0.0],
            [0.0, 1.0],
            [v.x, v.y],
        ])
    }
    
    #[inline]
    pub fn translate_to(p: Point2F) -> Matrix3x2F {
        Matrix3x2F::translation(Vector2F::new(p.x, p.y))
    }
    
    #[inline]
    pub fn scale(scale: SizeF, center: Point2F) -> Matrix3x2F {
        let trans = Vector2F::new(
            center.x - scale.width * center.x,
            center.y - scale.height * center.y,
        );
        
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
        let x = center.x;
        let y = center.y;
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
        let x = center.x;
        let y = center.y;
        
        Matrix3x2F::new([
            [    1.0,    tanx],
            [   tany,     1.0],
            [-y*tany, -x*tanx],
        ])
    }
    
    #[inline]
    pub fn determinant(&self) -> f32 {
        let [[a, b], [c, d], _] = self.matrix;
        
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
        let [[a, b], [c, d], [x, y]] = self.matrix;
        
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
        let [[a1, b1], [c1, d1], [x1, y1]] = self.matrix;
        let [[a2, b2], [c2, d2], [x2, y2]] = rhs.matrix;
        
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
        let [[a, b], [c, d], [x, y]] = rhs.matrix;
        let D2D1_POINT_2F { x: px, y: py } = self.0;
        
        Point2F::new(x + a*px + c*py, y + b*px + d*py)
    }
}

impl Mul<Matrix3x2F> for Vector2F {
    type Output = Vector2F;
    
    #[inline]
    fn mul(self, rhs: Matrix3x2F) -> Vector2F {
        let [[a, b], [c, d], _] = rhs.matrix;
        let D2D_VECTOR_2F { x: px, y: py } = self.0;
        
        Vector2F::new(a*px + c*py, b*px + d*py)
    }
}

impl PartialEq for Matrix3x2F {
    #[inline]
    fn eq(&self, rhs: &Matrix3x2F) -> bool {
        self.matrix == rhs.matrix
    }
}

impl BezierSegment {
    #[inline]
    pub fn new(p1: Point2F, p2: Point2F, p3: Point2F) -> BezierSegment {
        BezierSegment(D2D1_BEZIER_SEGMENT {
            point1: p1.0,
            point2: p2.0,
            point3: p3.0,
        })
    }
}

impl QuadBezierSegment {
    #[inline]
    pub fn new(p1: Point2F, p2: Point2F) -> QuadBezierSegment {
        QuadBezierSegment(D2D1_QUADRATIC_BEZIER_SEGMENT {
            point1: p1.0,
            point2: p2.0,
        })
    }
}

impl ArcSegment {
    #[inline]
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
    #[inline]
    pub fn new_cc_sm(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::CounterClockwise, ArcSize::Small)
    }
    
    /// Create a counter-clockwise large arc
    #[inline]
    pub fn new_cc_lg(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::CounterClockwise, ArcSize::Large)
    }
    
    
    /// Create a clockwise small arc
    #[inline]
    pub fn new_cw_sm(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::Clockwise, ArcSize::Small)
    }
    
    
    /// Create a counter-clockwise small arc
    #[inline]
    pub fn new_cw_lg(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(point, size, angle, SweepDirection::Clockwise, ArcSize::Large)
    }
}

impl BrushProperties {
    #[inline]
    pub fn new(opacity: f32, transform: &Matrix3x2F) -> BrushProperties {
        BrushProperties(D2D1_BRUSH_PROPERTIES {
            opacity: opacity,
            transform: transform.0,
        })
    }
    
    #[inline]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
    
    #[inline]
    pub fn transform(mut self, transform: &Matrix3x2F) -> Self {
        self.transform = transform.0;
        self
    }
}

impl Default for BrushProperties {
    #[inline]
    fn default() -> BrushProperties {
        BrushProperties::new(1.0, &Matrix3x2F::identity())
    }
}
