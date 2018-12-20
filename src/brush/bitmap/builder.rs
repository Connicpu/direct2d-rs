use crate::brush::bitmap::BitmapBrush;
use crate::enums::{BitmapInterpolationMode, ExtendMode};
use crate::error::D2DResult;
use crate::image::Bitmap;
use crate::properties::BrushProperties;
use crate::render_target::RenderTarget;

use std::ptr;

use com_wrapper::ComWrapper;
use math2d::Matrix3x2f;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::D2D1_BITMAP_BRUSH_PROPERTIES;

pub struct BitmapBrushBuilder<'a> {
    context: &'a RenderTarget,
    bitmap: Option<&'a Bitmap>,
    b_properties: D2D1_BITMAP_BRUSH_PROPERTIES,
    properties: BrushProperties,
}

impl<'a> BitmapBrushBuilder<'a> {
    #[inline]
    pub fn new(context: &'a RenderTarget) -> Self {
        BitmapBrushBuilder {
            context,
            bitmap: None,
            b_properties: D2D1_BITMAP_BRUSH_PROPERTIES {
                extendModeX: ExtendMode::Clamp as u32,
                extendModeY: ExtendMode::Clamp as u32,
                interpolationMode: BitmapInterpolationMode::Linear as u32,
            },
            properties: BrushProperties::new(1.0, &Matrix3x2f::IDENTITY),
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<BitmapBrush> {
        let bitmap = self.bitmap.expect("`bitmap` must be specified");
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.context.rt().CreateBitmapBrush(
                bitmap.get_raw(),
                &self.b_properties,
                (&self.properties) as *const _ as *const _,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(BitmapBrush::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn with_bitmap(mut self, bitmap: &'a Bitmap) -> Self {
        self.bitmap = Some(bitmap);
        self
    }

    #[inline]
    pub fn with_extend_mode_x(mut self, mode: ExtendMode) -> Self {
        self.b_properties.extendModeX = mode as u32;
        self
    }

    #[inline]
    pub fn with_extend_mode_y(mut self, mode: ExtendMode) -> Self {
        self.b_properties.extendModeY = mode as u32;
        self
    }

    #[inline]
    pub fn with_properties(mut self, properties: BrushProperties) -> Self {
        self.properties = properties;
        self
    }

    #[inline]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.properties.opacity = opacity;
        self
    }

    #[inline]
    pub fn with_transform(mut self, transform: Matrix3x2f) -> Self {
        self.properties.transform = transform;
        self
    }
}
