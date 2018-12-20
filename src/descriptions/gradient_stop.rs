use math2d::Color;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

impl From<(f32, Color)> for GradientStop {
    #[inline]
    fn from((position, color): (f32, Color)) -> Self {
        GradientStop { position, color }
    }
}
