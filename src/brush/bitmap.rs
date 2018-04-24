use enums::{BitmapInterpolationMode, ExtendMode};
use error::D2DResult;
use image::Bitmap;
use math::{BrushProperties, Matrix3x2F};
use render_target::RenderTarget;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_BITMAP_BRUSH_PROPERTIES, ID2D1BitmapBrush};
use wio::com::ComPtr;

#[derive(Clone)]
pub struct BitmapBrush {
    ptr: ComPtr<ID2D1BitmapBrush>,
}

impl BitmapBrush {
    pub fn create<'a, R>(context: &'a R) -> BitmapBrushBuilder<'a, R>
    where
        R: RenderTarget + 'a,
    {
        BitmapBrushBuilder::new(context)
    }
}

brush_type!(BitmapBrush: ID2D1BitmapBrush);

pub struct BitmapBrushBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    context: &'a R,
    bitmap: Option<&'a Bitmap>,
    b_properties: D2D1_BITMAP_BRUSH_PROPERTIES,
    properties: BrushProperties,
}

impl<'a, R> BitmapBrushBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    pub fn new(context: &'a R) -> Self {
        BitmapBrushBuilder {
            context,
            bitmap: None,
            b_properties: D2D1_BITMAP_BRUSH_PROPERTIES {
                extendModeX: ExtendMode::Clamp as u32,
                extendModeY: ExtendMode::Clamp as u32,
                interpolationMode: BitmapInterpolationMode::Linear as u32,
            },
            properties: BrushProperties::new(1.0, &Matrix3x2F::IDENTITY),
        }
    }

    pub fn build(self) -> D2DResult<BitmapBrush> {
        let bitmap = self.bitmap.expect("`bitmap` must be specified");
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.context.rt().CreateBitmapBrush(
                bitmap.get_raw(),
                &self.b_properties,
                &self.properties.0,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(BitmapBrush::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline(always)]
    pub fn with_bitmap(mut self, bitmap: &'a Bitmap) -> Self {
        self.bitmap = Some(bitmap);
        self
    }

    #[inline(always)]
    pub fn with_extend_mode_x(mut self, mode: ExtendMode) -> Self {
        self.b_properties.extendModeX = mode as u32;
        self
    }

    #[inline(always)]
    pub fn with_extend_mode_y(mut self, mode: ExtendMode) -> Self {
        self.b_properties.extendModeY = mode as u32;
        self
    }

    #[inline(always)]
    pub fn with_properties(mut self, properties: BrushProperties) -> Self {
        self.properties = properties;
        self
    }

    #[inline(always)]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.properties.0.opacity = opacity;
        self
    }

    #[inline(always)]
    pub fn with_transform(mut self, transform: Matrix3x2F) -> Self {
        self.properties.0.transform = transform.0;
        self
    }
}
