use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul};
use winapi::*;
use helpers::EPSILON;

#[derive(Copy, Clone, Debug)]
pub struct Point2F(pub D2D1_POINT_2F);
#[derive(Copy, Clone, Debug)]
pub struct Vector2F(pub D2D_VECTOR_2F);
#[derive(Copy, Clone, Debug)]
pub struct SizeF(pub D2D1_SIZE_F);
#[derive(Copy, Clone, Debug)]
pub struct RectF(pub D2D1_RECT_F);
#[derive(Copy, Clone, Debug)]
pub struct RoundedRect(pub D2D1_ROUNDED_RECT);
#[derive(Copy, Clone, Debug)]
pub struct Ellipse(pub D2D1_ELLIPSE);
#[derive(Copy, Clone, Debug)]
pub struct Matrix3x2F(pub D2D1_MATRIX_3X2_F);
#[derive(Copy, Clone, Debug)]
pub struct BezierSegment(pub D2D1_BEZIER_SEGMENT);

impl Point2F {
    #[inline]
    pub fn new(x: f32, y: f32) -> Point2F {
        Point2F(D2D1_POINT_2F {
            x: x,
            y: y,
        })
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
        self == Matrix3x2F::identity()
    }
    
    
}

impl Mul for Matrix3x2F {
    fn mul(self, rhs: Matrix3x2F) -> Matrix3x2F {
        let [[a1, b1], [c1, d1], [x1, y1]] = self.0.matrix;
        let [[a2, b2], [c2, d2], [x2, y2]] = rhs.0.matrix;
        Matrix3x2F::new([
            [a1 * a2 + b1 * c2, a1 * b2 + b1 * d2],
            
        ])
    }
}

impl PartialEq for Matrix3x2F {
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
