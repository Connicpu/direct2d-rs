use crate::enums::{AlphaMode, FeatureLevel, PresentOptions, RenderTargetType, RenderTargetUsage};
use crate::error::D2DResult;
use crate::factory::Factory;
use crate::render_target::hwnd::HwndRenderTarget;

use com_wrapper::ComWrapper;
use dxgi::enums::Format;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::D2D1_HWND_RENDER_TARGET_PROPERTIES;
use winapi::um::d2d1::D2D1_RENDER_TARGET_PROPERTIES;
use winapi::um::dcommon::D2D1_PIXEL_FORMAT;
use winapi::um::dcommon::D2D1_SIZE_U;

pub struct HwndRenderTargetBuilder<'a> {
    factory: &'a Factory,
    rt_props: D2D1_RENDER_TARGET_PROPERTIES,
    hwnd_props: D2D1_HWND_RENDER_TARGET_PROPERTIES,
}

const DEFAULT_PROPS: D2D1_RENDER_TARGET_PROPERTIES = D2D1_RENDER_TARGET_PROPERTIES {
    _type: RenderTargetType::Hardware as u32,
    pixelFormat: D2D1_PIXEL_FORMAT {
        format: Format::B8G8R8A8Unorm as u32,
        alphaMode: AlphaMode::Premultiplied as u32,
    },
    dpiX: 0.0,
    dpiY: 0.0,
    usage: 0,
    minLevel: 0,
};

const DEFAULT_HWND_PROPS: D2D1_HWND_RENDER_TARGET_PROPERTIES = D2D1_HWND_RENDER_TARGET_PROPERTIES {
    hwnd: std::ptr::null_mut(),
    pixelSize: D2D1_SIZE_U {
        width: 0,
        height: 0,
    },
    presentOptions: 0,
};

impl<'a> HwndRenderTargetBuilder<'a> {
    #[inline]
    pub fn new(factory: &'a Factory) -> Self {
        HwndRenderTargetBuilder {
            factory,
            rt_props: DEFAULT_PROPS,
            hwnd_props: DEFAULT_HWND_PROPS,
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<HwndRenderTarget> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*self.factory.get_raw()).CreateHwndRenderTarget(
                &self.rt_props,
                &self.hwnd_props,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(HwndRenderTarget::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn with_target_type(mut self, target_type: RenderTargetType) -> Self {
        self.rt_props._type = target_type as u32;
        self
    }

    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.rt_props.pixelFormat.format = format as u32;
        self
    }

    #[inline]
    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.rt_props.pixelFormat.alphaMode = alpha_mode as u32;
        self
    }

    #[inline]
    pub fn with_dpi(mut self, dpi_x: f32, dpi_y: f32) -> Self {
        self.rt_props.dpiX = dpi_x;
        self.rt_props.dpiY = dpi_y;
        self
    }

    #[inline]
    pub fn with_usage(mut self, usage: RenderTargetUsage) -> Self {
        self.rt_props.usage = usage.0;
        self
    }

    #[inline]
    pub fn with_feature_level(mut self, level: FeatureLevel) -> Self {
        self.rt_props.minLevel = level as u32;
        self
    }

    #[inline]
    pub fn with_hwnd(mut self, hwnd: HWND) -> Self {
        self.hwnd_props.hwnd = hwnd;
        self
    }

    #[inline]
    pub fn with_pixel_size(mut self, width: u32, height: u32) -> Self {
        self.hwnd_props.pixelSize.width = width;
        self.hwnd_props.pixelSize.height = height;
        self
    }

    #[inline]
    pub fn with_present_options(mut self, options: PresentOptions) -> Self {
        self.hwnd_props.presentOptions = options.0;
        self
    }
}
