use crate::enums::AlphaMode;

use checked_enum::UncheckedEnum;
use dxgi::enums::Format;
use winapi::um::dcommon::D2D1_PIXEL_FORMAT;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PixelFormat {
    pub format: UncheckedEnum<Format>,
    pub alpha_mode: UncheckedEnum<AlphaMode>,
}

impl From<PixelFormat> for D2D1_PIXEL_FORMAT {
    fn from(format: PixelFormat) -> Self {
        unsafe { std::mem::transmute(format) }
    }
}

impl From<D2D1_PIXEL_FORMAT> for PixelFormat {
    fn from(format: D2D1_PIXEL_FORMAT) -> Self {
        unsafe { std::mem::transmute(format) }
    }
}
