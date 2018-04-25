use enums::{CapStyle, DashStyle, LineJoin, StrokeTransformType};
use error::D2DResult;
use factory::Factory;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1_1::{D2D1_STROKE_STYLE_PROPERTIES1, ID2D1StrokeStyle1};
use wio::com::ComPtr;

#[derive(Clone)]
pub struct StrokeStyle {
    ptr: ComPtr<ID2D1StrokeStyle1>,
}

impl StrokeStyle {
    #[inline]
    pub fn create<'a>(factory: &'a Factory) -> StrokeStyleBuilder<'a> {
        StrokeStyleBuilder::new(factory)
    }

    #[inline]
    pub fn get_start_cap(&self) -> CapStyle {
        unsafe { CapStyle::from_u32(self.ptr.GetStartCap()).unwrap() }
    }

    #[inline]
    pub fn get_end_cap(&self) -> CapStyle {
        unsafe { CapStyle::from_u32(self.ptr.GetEndCap()).unwrap() }
    }

    #[inline]
    pub fn get_dash_cap(&self) -> CapStyle {
        unsafe { CapStyle::from_u32(self.ptr.GetDashCap()).unwrap() }
    }

    #[inline]
    pub fn get_miter_limit(&self) -> f32 {
        unsafe { self.ptr.GetMiterLimit() }
    }

    #[inline]
    pub fn get_line_join(&self) -> LineJoin {
        unsafe { LineJoin::from_u32(self.ptr.GetLineJoin()).unwrap() }
    }

    #[inline]
    pub fn get_dash_offset(&self) -> f32 {
        unsafe { self.ptr.GetDashOffset() }
    }

    #[inline]
    pub fn get_dash_style(&self) -> DashStyle {
        unsafe { DashStyle::from_u32(self.ptr.GetDashStyle()).unwrap() }
    }

    #[inline]
    pub fn get_dashes_count(&self) -> u32 {
        unsafe { self.ptr.GetDashesCount() }
    }

    #[inline]
    pub fn get_dashes(&self) -> Vec<f32> {
        let count = self.get_dashes_count();
        let mut data = vec![0.0; count as usize];
        unsafe {
            self.ptr.GetDashes(data.as_mut_ptr(), count);
        }
        data
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1StrokeStyle1 {
        self.ptr.as_raw()
    }

    #[inline]
    pub unsafe fn from_raw(raw: *mut ID2D1StrokeStyle1) -> Self {
        StrokeStyle {
            ptr: ComPtr::from_raw(raw),
        }
    }
}

pub struct StrokeStyleBuilder<'a> {
    factory: &'a Factory,
    start_cap: CapStyle,
    end_cap: CapStyle,
    dash_cap: CapStyle,
    line_join: LineJoin,
    miter_limit: f32,
    dash_style: DashStyle,
    dash_offset: f32,
    transform_type: StrokeTransformType,
    dashes: Option<&'a [f32]>,
}

impl<'a> StrokeStyleBuilder<'a> {
    pub fn new(factory: &'a Factory) -> Self {
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
            transform_type: StrokeTransformType::Normal,
            dashes: None,
        }
    }

    pub fn build(self) -> D2DResult<StrokeStyle> {
        unsafe {
            let properties = self.to_d2d1();
            let (dashes, dash_count) = self.dashes
                .map(|d| (d.as_ptr(), d.len() as u32))
                .unwrap_or((ptr::null(), 0));

            let mut ptr = ptr::null_mut();
            let hr = (*self.factory.get_raw()).CreateStrokeStyle(
                &properties,
                dashes,
                dash_count,
                &mut ptr,
            );

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

    pub fn with_transform_type(mut self, transform_type: StrokeTransformType) -> Self {
        self.transform_type = transform_type;
        self
    }

    pub fn with_dashes(mut self, dashes: &'a [f32]) -> Self {
        self.dash_style = DashStyle::Custom;
        self.dashes = Some(dashes);
        self
    }

    fn to_d2d1(&self) -> D2D1_STROKE_STYLE_PROPERTIES1 {
        D2D1_STROKE_STYLE_PROPERTIES1 {
            startCap: self.start_cap as u32,
            endCap: self.end_cap as u32,
            dashCap: self.dash_cap as u32,
            lineJoin: self.line_join as u32,
            miterLimit: self.miter_limit,
            dashStyle: self.dash_style as u32,
            dashOffset: self.dash_offset,
            transformType: self.transform_type as u32,
        }
    }
}
