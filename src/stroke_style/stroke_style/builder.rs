use crate::enums::{CapStyle, DashStyle, LineJoin};
use crate::factory::IFactory;
use crate::stroke_style::StrokeStyle;

use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::D2D1_STROKE_STYLE_PROPERTIES;

pub struct StrokeStyleBuilder<'a> {
    factory: &'a dyn IFactory,
    start_cap: CapStyle,
    end_cap: CapStyle,
    dash_cap: CapStyle,
    line_join: LineJoin,
    miter_limit: f32,
    dash_style: DashStyle,
    dash_offset: f32,
    dashes: Option<&'a [f32]>,
}

impl<'a> StrokeStyleBuilder<'a> {
    pub fn new(factory: &'a dyn IFactory) -> Self {
        // default values taken from D2D1::StrokeStyleProperties in d2d1helper.h
        StrokeStyleBuilder {
            factory,
            start_cap: CapStyle::Flat,
            end_cap: CapStyle::Flat,
            dash_cap: CapStyle::Flat,
            line_join: LineJoin::Miter,
            miter_limit: 10.0,
            dash_style: DashStyle::Solid,
            dash_offset: 0.0,
            dashes: None,
        }
    }

    pub fn build(self) -> Result<StrokeStyle, Error> {
        unsafe {
            let properties = self.to_d2d1();
            let (dashes, dash_count) = self
                .dashes
                .map(|d| (d.as_ptr(), d.len() as u32))
                .unwrap_or((std::ptr::null(), 0));

            let mut ptr = std::ptr::null_mut();
            let hr =
                self.factory
                    .raw_f()
                    .CreateStrokeStyle(&properties, dashes, dash_count, &mut ptr);

            if SUCCEEDED(hr) {
                Ok(StrokeStyle::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn with_start_cap(mut self, start_cap: CapStyle) -> Self {
        self.start_cap = start_cap;
        self
    }

    pub fn with_end_cap(mut self, end_cap: CapStyle) -> Self {
        self.end_cap = end_cap;
        self
    }

    pub fn with_dash_cap(mut self, dash_cap: CapStyle) -> Self {
        self.dash_cap = dash_cap;
        self
    }

    pub fn with_line_join(mut self, line_join: LineJoin) -> Self {
        self.line_join = line_join;
        self
    }

    pub fn with_miter_limit(mut self, miter_limit: f32) -> Self {
        self.miter_limit = miter_limit;
        self
    }

    pub fn with_dash_style(mut self, dash_style: DashStyle) -> Self {
        self.dash_style = dash_style;
        self
    }

    pub fn with_dash_offset(mut self, dash_offset: f32) -> Self {
        self.dash_offset = dash_offset;
        self
    }

    pub fn with_dashes(mut self, dashes: &'a [f32]) -> Self {
        self.dash_style = DashStyle::Custom;
        self.dashes = Some(dashes);
        self
    }

    fn to_d2d1(&self) -> D2D1_STROKE_STYLE_PROPERTIES {
        D2D1_STROKE_STYLE_PROPERTIES {
            startCap: self.start_cap as u32,
            endCap: self.end_cap as u32,
            dashCap: self.dash_cap as u32,
            lineJoin: self.line_join as u32,
            miterLimit: self.miter_limit,
            dashStyle: self.dash_style as u32,
            dashOffset: self.dash_offset,
        }
    }
}
