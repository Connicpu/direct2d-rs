use comptr::ComPtr;
use error::D2D1Error;
use helpers::{FromRaw, GetRaw};

use winapi::um::d2d1::*;
use winapi::um::d2d1_1::*;

#[derive(Clone, Debug)]
pub struct StrokeStyle {
    stroke: ComPtr<ID2D1StrokeStyle1>,
}

#[derive(Copy, Clone, Debug)]
pub struct StrokeStyleProperties<'a> {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_style: DashStyle,
    pub dash_offset: f32,
    pub dashes: Option<&'a [f32]>,
}

impl<'a> StrokeStyleProperties<'a> {
    pub fn new() -> StrokeStyleProperties<'static> {
        // default values taken from D2D1::StrokeStyleProperties in d2d1helper.h
        StrokeStyleProperties {
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

    pub unsafe fn get_d2d1_data(&self) -> D2D1_STROKE_STYLE_PROPERTIES1 {
        D2D1_STROKE_STYLE_PROPERTIES1 {
            startCap: self.start_cap as u32,
            endCap: self.end_cap as u32,
            dashCap: self.dash_cap as u32,
            lineJoin: self.line_join as u32,
            miterLimit: self.miter_limit,
            dashStyle: self.dash_style as u32,
            dashOffset: self.dash_offset,
            transformType: D2D1_STROKE_TRANSFORM_TYPE_NORMAL,
        }
    }
}

impl<'a> Default for StrokeStyleProperties<'a> {
    fn default() -> StrokeStyleProperties<'static> {
        StrokeStyleProperties::new()
    }
}

impl StrokeStyle {
    pub unsafe fn get_ptr(&self) -> *mut ID2D1StrokeStyle1 {
        let ptr = self.stroke.raw_value();
        assert!(!ptr.is_null());
        ptr
    }

    pub fn get_start_cap(&self) -> Result<CapStyle, D2D1Error> {
        unsafe { CapStyle::from_raw((*self.get_ptr()).GetStartCap()) }
    }

    pub fn get_end_cap(&self) -> Result<CapStyle, D2D1Error> {
        unsafe { CapStyle::from_raw((*self.get_ptr()).GetEndCap()) }
    }

    pub fn get_dash_cap(&self) -> Result<CapStyle, D2D1Error> {
        unsafe { CapStyle::from_raw((*self.get_ptr()).GetDashCap()) }
    }

    pub fn get_miter_limit(&self) -> f32 {
        unsafe { (*self.get_ptr()).GetMiterLimit() }
    }

    pub fn get_line_join(&self) -> Result<LineJoin, D2D1Error> {
        unsafe { LineJoin::from_raw((*self.get_ptr()).GetLineJoin()) }
    }

    pub fn get_dash_offset(&self) -> f32 {
        unsafe { (*self.get_ptr()).GetDashOffset() }
    }

    pub fn get_dash_style(&self) -> Result<DashStyle, D2D1Error> {
        unsafe { DashStyle::from_raw((*self.get_ptr()).GetDashStyle()) }
    }

    pub fn get_dashes_count(&self) -> u32 {
        unsafe { (*self.get_ptr()).GetDashesCount() }
    }

    pub fn get_dashes(&self) -> Vec<f32> {
        let count = self.get_dashes_count();
        let mut data = vec![0.0; count as usize];
        unsafe {
            (*self.get_ptr()).GetDashes(data.as_mut_ptr(), count);
        }
        data
    }
}

impl GetRaw for StrokeStyle {
    type Raw = ID2D1StrokeStyle1;
    unsafe fn get_raw(&self) -> *mut ID2D1StrokeStyle1 {
        self.stroke.raw_value()
    }
}

impl FromRaw for StrokeStyle {
    type Raw = ID2D1StrokeStyle1;
    unsafe fn from_raw(raw: *mut ID2D1StrokeStyle1) -> Self {
        StrokeStyle {
            stroke: ComPtr::from_existing(raw),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CapStyle {
    Flat = 0,
    Square = 1,
    Round = 2,
    Triangle = 3,
}

impl CapStyle {
    fn from_raw(value: D2D1_CAP_STYLE) -> Result<CapStyle, D2D1Error> {
        use self::CapStyle::*;
        match value {
            D2D1_CAP_STYLE_FLAT => Ok(Flat),
            D2D1_CAP_STYLE_SQUARE => Ok(Square),
            D2D1_CAP_STYLE_ROUND => Ok(Round),
            D2D1_CAP_STYLE_TRIANGLE => Ok(Triangle),
            _ => Err(D2D1Error::UnknownEnumValue),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LineJoin {
    Miter = 0,
    Bevel = 1,
    Round = 2,
    MiterOrBevel = 3,
}

impl LineJoin {
    fn from_raw(value: D2D1_LINE_JOIN) -> Result<LineJoin, D2D1Error> {
        use self::LineJoin::*;
        match value {
            D2D1_LINE_JOIN_MITER => Ok(Miter),
            D2D1_LINE_JOIN_BEVEL => Ok(Bevel),
            D2D1_LINE_JOIN_ROUND => Ok(Round),
            D2D1_LINE_JOIN_MITER_OR_BEVEL => Ok(MiterOrBevel),
            _ => Err(D2D1Error::UnknownEnumValue),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DashStyle {
    Solid = 0,
    Dash = 1,
    Dot = 2,
    DashDot = 3,
    DashDotDot = 4,
    Custom = 5,
}

impl DashStyle {
    fn from_raw(value: D2D1_DASH_STYLE) -> Result<DashStyle, D2D1Error> {
        use self::DashStyle::*;
        match value {
            D2D1_DASH_STYLE_SOLID => Ok(Solid),
            D2D1_DASH_STYLE_DASH => Ok(Dash),
            D2D1_DASH_STYLE_DOT => Ok(Dot),
            D2D1_DASH_STYLE_DASH_DOT => Ok(DashDot),
            D2D1_DASH_STYLE_DASH_DOT_DOT => Ok(DashDotDot),
            D2D1_DASH_STYLE_CUSTOM => Ok(Custom),
            _ => Err(D2D1Error::UnknownEnumValue),
        }
    }
}
