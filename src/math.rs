use winapi::*;

pub struct Point2F(pub D2D1_POINT_2F);
pub struct Vector2F(pub D2D_VECTOR_2F);
pub struct SizeF(pub D2D1_SIZE_F);
pub struct RectF(pub D2D1_RECT_F);
pub struct RoundedRect(pub D2D1_ROUNDED_RECT);
pub struct Ellipse(pub D2D1_ELLIPSE);
pub struct Matrix3x2F(pub D2D1_MATRIX_3X2_F);

impl Point2F {
    pub fn new(x: f32, y: f32) -> Point2F {
        Point2F(D2D1_POINT_2F {
            x: x,
            y: y,
        })
    }
}

impl RectF {
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
    pub fn new(data: [[f32; 2]; 3]) -> Matrix3x2F {
        Matrix3x2F(D2D1_MATRIX_3X2_F {
            matrix: data,
        })
    }
}
