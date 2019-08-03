use crate::brush::solid_color::SolidColorBrush;
use crate::properties::BrushProperties;
use crate::render_target::IRenderTarget;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::{Color, Matrix3x2f};
use winapi::shared::winerror::SUCCEEDED;

pub struct SolidColorBrushBuilder<'a> {
    context: &'a dyn IRenderTarget,
    properties: BrushProperties,
    color: Option<Color>,
}

impl<'a> SolidColorBrushBuilder<'a> {
    #[inline]
    pub fn new(context: &'a dyn IRenderTarget) -> Self {
        SolidColorBrushBuilder {
            context,
            properties: BrushProperties::new(1.0, &Matrix3x2f::IDENTITY),
            color: None,
        }
    }

    #[inline]
    pub fn build(self) -> Result<SolidColorBrush, Error> {
        let color = self.color.expect("`color` must be specified");
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.context.raw_rt().CreateSolidColorBrush(
                (&color) as *const _ as *const _,
                (&self.properties) as *const _ as *const _,
                &mut ptr,
            );
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
        C: Into<Color>,
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
        self.properties.opacity = opacity;
        self
    }

    #[inline]
    pub fn with_transform(mut self, transform: Matrix3x2f) -> Self {
        self.properties.transform = transform;
        self
    }
}
