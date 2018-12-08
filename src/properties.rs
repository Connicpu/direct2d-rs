use enums::AlphaMode;

use math2d::Matrix3x2f;
use math2d::Point2f;
use dxgi::enums::Format;
use winapi::um::d2d1::{
    D2D1_BITMAP_PROPERTIES, D2D1_BRUSH_PROPERTIES, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
    D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
};
use winapi::um::dcommon::D2D1_PIXEL_FORMAT;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PixelFormat {
    pub format: Format,
    pub alpha_mode: AlphaMode,
}

impl From<PixelFormat> for D2D1_PIXEL_FORMAT {
    #[inline]
    fn from(pf: PixelFormat) -> Self {
        D2D1_PIXEL_FORMAT {
            format: pf.format as u32,
            alphaMode: pf.alpha_mode as u32,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BitmapProperties {
    pub pixel_format: PixelFormat,
    pub dpi_x: f32,
    pub dpi_y: f32,
}

impl From<BitmapProperties> for D2D1_BITMAP_PROPERTIES {
    #[inline]
    fn from(bp: BitmapProperties) -> Self {
        D2D1_BITMAP_PROPERTIES {
            pixelFormat: bp.pixel_format.into(),
            dpiX: bp.dpi_x,
            dpiY: bp.dpi_y,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BrushProperties {
    pub opacity: f32,
    pub transform: Matrix3x2f,
}

impl BrushProperties {
    #[inline]
    pub fn new(opacity: f32, transform: &Matrix3x2f) -> BrushProperties {
        BrushProperties {
            opacity,
            transform: *transform,
        }
    }
}

impl From<BrushProperties> for D2D1_BRUSH_PROPERTIES {
    #[inline]
    fn from(bp: BrushProperties) -> D2D1_BRUSH_PROPERTIES {
        D2D1_BRUSH_PROPERTIES {
            opacity: bp.opacity,
            transform: bp.transform.into(),
        }
    }
}

impl Default for BrushProperties {
    #[inline]
    fn default() -> BrushProperties {
        BrushProperties {
            opacity: 1.0,
            transform: Matrix3x2f::IDENTITY,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct LinearGradientBrushProperties {
    pub start: Point2f,
    pub end: Point2f,
}

impl From<LinearGradientBrushProperties> for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    #[inline]
    fn from(bp: LinearGradientBrushProperties) -> D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
        D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
            startPoint: bp.start.into(),
            endPoint: bp.end.into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RadialGradientBrushProperties {
    pub center: Point2f,
    pub origin_offset: Point2f,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl From<RadialGradientBrushProperties> for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    #[inline]
    fn from(bp: RadialGradientBrushProperties) -> D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
        D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
            center: bp.center.into(),
            gradientOriginOffset: bp.origin_offset.into(),
            radiusX: bp.radius_x,
            radiusY: bp.radius_y,
        }
    }
}
