use crate::brush::solid_color::SolidColorBrush;
use crate::error::D2DResult;
use crate::properties::BrushProperties;
use crate::render_target::RenderTarget;

use std::ptr;

use com_wrapper::ComWrapper;
use math2d::{Color, Matrix3x2f};
use winapi::shared::winerror::SUCCEEDED;

pub struct SolidColorBrushBuilder<'a> {
    context: &'a RenderTarget,
    properties: BrushProperties,
    color: Option<Color>,
}

impl<'a> SolidColorBrushBuilder<'a> {
    #[inline]
    pub fn new(context: &'a RenderTarget) -> Self {
        SolidColorBrushBuilder {
            context,
            properties: BrushProperties::new(1.0, &Matrix3x2f::IDENTITY),
            color: None,
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<SolidColorBrush> {
        let color = self.color.expect("`color` must be specified");
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.context.rt().CreateSolidColorBrush(
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
