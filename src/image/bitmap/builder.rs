use crate::enums::AlphaMode;
use crate::error::D2DResult;
use crate::image::bitmap::{Bitmap, SharedBitmapSource};
use crate::render_target::RenderTarget;

use com_wrapper::ComWrapper;
use dcommon::Error;
use dxgi::enums::Format;
use math2d::Sizeu;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::IID;
use winapi::um::d2d1::D2D1_BITMAP_PROPERTIES;
use winapi::um::dcommon::D2D1_PIXEL_FORMAT;
use winapi::um::unknwnbase::IUnknown;

pub struct BitmapBuilder<'a> {
    context: &'a RenderTarget,
    source: Option<BitmapSource<'a>>,
    properties: D2D1_BITMAP_PROPERTIES,
}

const DEFAULT_BITMAP_PROPS: D2D1_BITMAP_PROPERTIES = D2D1_BITMAP_PROPERTIES {
    pixelFormat: D2D1_PIXEL_FORMAT {
        format: Format::Unknown as u32,
        alphaMode: AlphaMode::Premultiplied as u32,
    },
    dpiX: 96.0,
    dpiY: 96.0,
};

impl<'a> BitmapBuilder<'a> {
    #[inline]
    pub fn new(context: &'a RenderTarget) -> Self {
        BitmapBuilder {
            context,
            source: None,
            properties: DEFAULT_BITMAP_PROPS,
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<Bitmap> {
        let source = self.source.expect("An image source must be specified");
        let properties = &self.properties;
        match source {
            BitmapSource::Raw {
                source,
                pitch,
                size,
            } => unsafe {
                let size = size.into();
                let src = match source {
                    Some(slice) => slice.as_ptr() as *const c_void,
                    None => std::ptr::null(),
                };

                let mut ptr = std::ptr::null_mut();
                let hr = self
                    .context
                    .rt()
                    .CreateBitmap(size, src, pitch, properties, &mut ptr);

                Error::map_if(hr, || Bitmap::from_raw(ptr))
            },
            BitmapSource::Shared(ref riid, data) => unsafe {
                let mut ptr = std::ptr::null_mut();
                let hr = self.context.rt().CreateSharedBitmap(
                    riid,
                    data as *const _ as _,
                    properties,
                    &mut ptr,
                );

                Error::map_if(hr, || Bitmap::from_raw(ptr))
            },
            // BitmapSource::Wic(wic) => unimplemented!(),
        }
    }

    #[inline]
    pub fn with_blank_image(mut self, size: impl Into<Sizeu>) -> Self {
        let size = size.into();
        self.source = Some(BitmapSource::Raw {
            size,
            source: None,
            pitch: 0,
        });
        self
    }

    #[inline]
    pub fn with_raw_data(mut self, size: impl Into<Sizeu>, data: &'a [u8], pitch: u32) -> Self {
        let size = size.into();
        assert!(size.height as usize * pitch as usize <= data.len());
        self.source = Some(BitmapSource::Raw {
            size,
            source: Some(data),
            pitch,
        });
        self
    }

    #[inline]
    pub fn with_shared_data(mut self, data: &'a impl SharedBitmapSource) -> Self {
        let (iid, data) = data.get_shared_source();
        self.source = Some(BitmapSource::Shared(iid, data));
        self
    }

    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.properties.pixelFormat.format = format as u32;
        self
    }

    #[inline]
    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.properties.pixelFormat.alphaMode = alpha_mode as u32;
        self
    }

    #[inline]
    pub fn with_dpi(mut self, dpi_x: f32, dpi_y: f32) -> Self {
        self.properties.dpiX = dpi_x;
        self.properties.dpiY = dpi_y;
        self
    }
}

enum BitmapSource<'a> {
    Raw {
        source: Option<&'a [u8]>,
        pitch: u32,
        size: Sizeu,
    },
    Shared(IID, &'a IUnknown),
    // Wic(&'a wic::BitmapSource)
}
