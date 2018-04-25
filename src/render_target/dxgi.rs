use enums::{AlphaMode, FeatureLevel, RenderTargetType, RenderTargetUsage};
use error::D2DResult;
use factory::Factory;
use render_target::{GenericRenderTarget,RenderTarget};

use std::ptr;

use dxgi::Format;
use dxgi::surface::Surface as DxgiSurface;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_RENDER_TARGET_PROPERTIES, ID2D1RenderTarget};
use winapi::um::dcommon::D2D1_PIXEL_FORMAT;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct DxgiSurfaceRenderTarget {
    ptr: ComPtr<ID2D1RenderTarget>,
}

impl DxgiSurfaceRenderTarget {
    #[inline]
    pub fn create<'a>(factory: &'a Factory) -> DxgiSurfaceRenderTargetBuilder<'a> {
        DxgiSurfaceRenderTargetBuilder::new(factory)
    }

    #[inline]
    pub fn as_generic(&self) -> GenericRenderTarget {
        unsafe {
            let ptr = self.get_raw();
            (*ptr).AddRef();
            GenericRenderTarget::from_raw(ptr)
        }
    }

    #[inline]
    pub unsafe fn from_raw(raw: *mut ID2D1RenderTarget) -> Self {
        DxgiSurfaceRenderTarget {
            ptr: ComPtr::from_raw(raw),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1RenderTarget {
        self.ptr.as_raw()
    }
}

impl RenderTarget for DxgiSurfaceRenderTarget {
    #[inline]
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *self.ptr.as_raw()
    }
}

unsafe impl Send for DxgiSurfaceRenderTarget {}
unsafe impl Sync for DxgiSurfaceRenderTarget {}

pub struct DxgiSurfaceRenderTargetBuilder<'a> {
    factory: &'a Factory,
    surface: Option<&'a DxgiSurface>,
    rt_props: D2D1_RENDER_TARGET_PROPERTIES,
}

const DEFAULT_PROPS: D2D1_RENDER_TARGET_PROPERTIES = D2D1_RENDER_TARGET_PROPERTIES {
    _type: RenderTargetType::Hardware as u32,
    pixelFormat: D2D1_PIXEL_FORMAT {
        format: Format::Unknown as u32,
        alphaMode: AlphaMode::Premultiplied as u32,
    },
    dpiX: 0.0,
    dpiY: 0.0,
    usage: 0,
    minLevel: 0,
};

impl<'a> DxgiSurfaceRenderTargetBuilder<'a> {
    #[inline]
    pub fn new(factory: &'a Factory) -> Self {
        DxgiSurfaceRenderTargetBuilder {
            factory,
            surface: None,
            rt_props: DEFAULT_PROPS,
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<DxgiSurfaceRenderTarget> {
        let surface = self.surface.expect("`surface` must be specified");
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*self.factory.get_raw()).CreateDxgiSurfaceRenderTarget(
                surface.get_raw(),
                &self.rt_props,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(DxgiSurfaceRenderTarget::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn with_surface(mut self, surface: &'a DxgiSurface) -> Self {
        self.surface = Some(surface);
        self
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
}
