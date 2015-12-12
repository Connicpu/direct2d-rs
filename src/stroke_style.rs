use winapi::*;
use comptr::ComPtr;
use error::D2D1Error;

pub struct StrokeStyle {
    stroke: ComPtr<ID2D1StrokeStyle>,
}

impl StrokeStyle {
    pub unsafe fn get_ptr(&self) -> *mut ID2D1StrokeStyle {
        let ptr = self.stroke.raw_value();
        assert!(!ptr.is_null());
        ptr
    }
    
    pub fn get_start_cap(&self) -> Result<CapStyle, D2D1Error> {
        unsafe {
            CapStyle::from_raw((*self.get_ptr()).GetStartCap())
        }
    }
    
    pub fn get_end_cap(&self) -> Result<CapStyle, D2D1Error> {
        unsafe {
            CapStyle::from_raw((*self.get_ptr()).GetEndCap())
        }
    }
    
    pub fn get_dash_cap(&self) -> Result<CapStyle, D2D1Error> {
        unsafe {
            CapStyle::from_raw((*self.get_ptr()).GetDashCap())
        }
    }
    
    pub fn get_miter_limit(&self) -> f32 {
        unsafe {
            (*self.get_ptr()).GetMiterLimit()
        }
    }
    
    pub fn get_line_join(&self) -> Result<LineJoin, D2D1Error> {
        unsafe {
            LineJoin::from_raw((*self.get_ptr()).GetLineJoin())
        }
    }
}

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
