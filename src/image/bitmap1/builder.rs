use crate::descriptions::PixelFormat;
use crate::device_context::DeviceContext;
use crate::enums::{AlphaMode, BitmapOptions};
use crate::image::Bitmap1;
use crate::properties::BitmapProperties1;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use dxgi::enums::Format;
use math2d::Sizeu;

pub struct BitmapBuilder1<'a> {
    context: &'a DeviceContext,
    source: Option<Source<'a>>,
    properties: BitmapProperties1,
}

enum Source<'a> {
    Memory {
        size: Sizeu,
        data: &'a [u8],
        stride: u32,
    },
    Dxgi {
        dxgi: &'a dxgi::surface::Surface,
    },
}

impl<'a> BitmapBuilder1<'a> {
    pub(super) fn new(context: &'a DeviceContext) -> Self {
        BitmapBuilder1 {
            context,
            source: None,
            properties: BitmapProperties1 {
                pixel_format: PixelFormat {
                    format: Format::R8G8B8A8Unorm.into(),
                    alpha_mode: AlphaMode::Premultiplied.into(),
                },
                dpi_x: 96.0,
                dpi_y: 96.0,
                options: BitmapOptions::NONE,
            },
        }
    }

    pub fn build(self) -> Result<Bitmap1, Error> {
        let source = self.source.expect("One source must be specified");
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = match source {
                Source::Memory { size, data, stride } => (*self.context.get_raw()).CreateBitmap(
                    size.into(),
                    data.as_ptr() as _,
                    stride,
                    &self.properties.into(),
                    &mut ptr,
                ),
                Source::Dxgi { dxgi } => (*self.context.get_raw()).CreateBitmapFromDxgiSurface(
                    dxgi.get_raw(),
                    &self.properties.into(),
                    &mut ptr,
                ),
            };

            Error::map_if(hr, || Bitmap1::from_raw(ptr))
        }
    }

    #[inline]
    pub fn with_dxgi_surface(mut self, dxgi: &'a dxgi::surface::Surface) -> Self {
        self.source = Some(Source::Dxgi { dxgi });
        self
    }

    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.properties.pixel_format.format = format.into();
        self
    }

    #[inline]
    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.properties.pixel_format.alpha_mode = alpha_mode.into();
        self
    }

    #[inline]
    pub fn with_dpi(mut self, dpi_x: f32, dpi_y: f32) -> Self {
        self.properties.dpi_x = dpi_x;
        self.properties.dpi_y = dpi_y;
        self
    }

    #[inline]
    pub fn with_options(mut self, options: BitmapOptions) -> Self {
        self.properties.options = options;
        self
    }
}
