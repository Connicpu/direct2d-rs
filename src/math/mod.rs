use winapi::um::d2d1::*;

#[doc(inline)]
pub use math::color::ColorF;
#[doc(inline)]
pub use math::matrix::Matrix3x2F;
#[doc(inline)]
pub use math::point2f::Point2F;
#[doc(inline)]
pub use math::rectf::RectF;
#[doc(inline)]
pub use math::segments::{ArcSegment, BezierSegment, QuadBezierSegment};
#[doc(inline)]
pub use math::sizef::SizeF;
#[doc(inline)]
pub use math::sizeu::SizeU;
#[doc(inline)]
pub use math::thicknessf::ThicknessF;
#[doc(inline)]
pub use math::vector2f::Vector2F;

#[doc(hidden)]
pub mod color;
#[doc(hidden)]
pub mod matrix;
#[doc(hidden)]
pub mod point2f;
#[doc(hidden)]
pub mod rectf;
#[doc(hidden)]
pub mod segments;
#[doc(hidden)]
pub mod sizef;
#[doc(hidden)]
pub mod sizeu;
#[doc(hidden)]
pub mod thicknessf;
#[doc(hidden)]
pub mod vector2f;

pub mod debug;

math_wrappers! {
    /// Rounded rectangle descriptor.
    pub struct RoundedRect(pub D2D1_ROUNDED_RECT);

    /// Ellipse descriptor.
    pub struct Ellipse(pub D2D1_ELLIPSE);

    /// Basic properties that are passed to every brush when created.
    pub struct BrushProperties(pub D2D1_BRUSH_PROPERTIES);

    /// Properties for linear gradient brushes.
    pub struct LinearGradientBrushProperties(pub D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES);

    /// Properties for radial gradient brushes.
    pub struct RadialGradientBrushProperties(pub D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES);
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
