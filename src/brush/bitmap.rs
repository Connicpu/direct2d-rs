use enums::{BitmapInterpolationMode, ExtendMode, UncheckedEnum};
use error::D2DResult;
use image::Bitmap;
use math::{BrushProperties, Matrix3x2F};
use render_target::RenderTarget;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_BITMAP_BRUSH_PROPERTIES, ID2D1BitmapBrush};
use wio::com::ComPtr;

/// Paints an area with a bitmap.
#[derive(Clone)]
pub struct BitmapBrush {
    ptr: ComPtr<ID2D1BitmapBrush>,
}

impl BitmapBrush {
    /// Begins a builder for the bitmap brush.
    #[inline]
    pub fn create<'a, R>(context: &'a R) -> BitmapBrushBuilder<'a, R>
    where
        R: RenderTarget + 'a,
    {
        BitmapBrushBuilder::new(context)
    }

    /// Gets the bitmap currently drawin by this Brush.
    #[inline]
    pub fn get_bitmap(&self) -> Bitmap {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetBitmap(&mut ptr);
            assert!(!ptr.is_null());
            Bitmap::from_raw(ptr)
        }
    }

    /// Gets how the brush draws pixels outside the normal content area on the X axis.
    #[inline]
    pub fn get_extend_mode_x(&self) -> UncheckedEnum<ExtendMode> {
        unsafe { self.ptr.GetExtendModeX().into() }
    }

    /// Gets how the brush draws pixels outside the normal content area on the Y axis.
    #[inline]
    pub fn get_extend_mode_y(&self) -> UncheckedEnum<ExtendMode> {
        unsafe { self.ptr.GetExtendModeY().into() }
    }

    /// Gets the interpolation mode i.e. Nearest Neighbor vs Linear sampling.
    #[inline]
    pub fn get_interpolation_mode(&self) -> UncheckedEnum<BitmapInterpolationMode> {
        unsafe { self.ptr.GetInterpolationMode().into() }
    }

    /// Changes the bitmap drawn by this brush
    #[inline]
    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        unsafe { self.ptr.SetBitmap(bitmap.get_raw()) }
    }

    /// Sets how the brush draws pixels outside the normal content area on the X axis.
    #[inline]
    pub fn set_extend_mode_x(&self, mode: ExtendMode) {
        unsafe { self.ptr.SetExtendModeX(mode as u32) }
    }

    /// Sets how the brush draws pixels outside the normal content area on the Y axis.
    #[inline]
    pub fn set_extend_mode_y(&self, mode: ExtendMode) {
        unsafe { self.ptr.SetExtendModeY(mode as u32) }
    }

    /// Sets the interpolation mode i.e. Nearest Neighbor vs Linear sampling.
    #[inline]
    pub fn set_interpolation_mode(&self, mode: BitmapInterpolationMode) {
        unsafe { self.ptr.SetInterpolationMode(mode as u32) }
    }
}

brush_type!(BitmapBrush: ID2D1BitmapBrush);

/// Builder for creating bitmap brushes.
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
    /// Creates the builder with all of the properties set to the default.
    #[inline]
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

    /// Builds the bitmap. **Panics** you you didn't provide a bitmap!
    #[inline]
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

    /// Provides the bitmap which should be painted by this brush. No bitmap is provided by
    /// default.
    #[inline]
    pub fn with_bitmap(mut self, bitmap: &'a Bitmap) -> Self {
        self.bitmap = Some(bitmap);
        self
    }

    /// Sets how the brush draws pixels outside the normal content area on the X axis
    #[inline]
    pub fn with_extend_mode_x(mut self, mode: ExtendMode) -> Self {
        self.b_properties.extendModeX = mode as u32;
        self
    }

    /// Sets how the brush draws pixels outside the normal content area on the Y axis
    #[inline]
    pub fn with_extend_mode_y(mut self, mode: ExtendMode) -> Self {
        self.b_properties.extendModeY = mode as u32;
        self
    }

    #[inline]
    /// Provides standard bitmap properties (opacity and transform) if you've already created
    /// them into a BrushProperties struct.
    pub fn with_properties(mut self, properties: BrushProperties) -> Self {
        self.properties = properties;
        self
    }

    #[inline]
    /// Sets the opacity of the brush (default 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.properties.0.opacity = opacity;
        self
    }

    #[inline]
    /// Sets the transform of the brush (defaults to [Identity][1] matrix).
    /// 
    /// [1]: ../../math/struct.Matrix3x2F.html#associatedconstant.IDENTITY
    pub fn with_transform(mut self, transform: Matrix3x2F) -> Self {
        self.properties.0.transform = transform.0;
        self
    }
}
