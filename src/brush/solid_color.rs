use error::D2DResult;
use math::{BrushProperties, ColorF, Matrix3x2F};
use render_target::RenderTarget;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1SolidColorBrush;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct SolidColorBrush {
    ptr: ComPtr<ID2D1SolidColorBrush>,
}

impl SolidColorBrush {
    #[inline]
    pub fn create<'a, R>(context: &'a R) -> SolidColorBrushBuilder<'a, R>
    where
        R: RenderTarget + 'a,
    {
        SolidColorBrushBuilder::new(context)
    }

    #[inline]
    pub fn set_color(&mut self, color: &ColorF) {
        unsafe { self.ptr.SetColor(&color.0) };
    }

    #[inline]
    pub fn get_color(&self) -> ColorF {
        unsafe { ColorF(self.ptr.GetColor()) }
    }
}

brush_type!(SolidColorBrush: ID2D1SolidColorBrush);

pub struct SolidColorBrushBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    context: &'a R,
    properties: BrushProperties,
    color: Option<ColorF>,
}

impl<'a, R> SolidColorBrushBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    #[inline]
    pub fn new(context: &'a R) -> Self {
        SolidColorBrushBuilder {
            context,
            properties: BrushProperties::new(1.0, &Matrix3x2F::IDENTITY),
            color: None,
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<SolidColorBrush> {
        let color = self.color.expect("`color` must be specified");
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr =
                self.context
                    .rt()
                    .CreateSolidColorBrush(&color.0, &self.properties.0, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(SolidColorBrush::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn with_color<C>(mut self, color: C) -> Self
    where
        C: Into<ColorF>,
    {
        self.color = Some(color.into());
        self
    }

    #[inline]
    pub fn with_properties(mut self, properties: BrushProperties) -> Self {
        self.properties = properties;
        self
    }

    #[inline]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.properties.0.opacity = opacity;
        self
    }

    #[inline]
    pub fn with_transform(mut self, transform: Matrix3x2F) -> Self {
        self.properties.0.transform = transform.0;
        self
    }
}
