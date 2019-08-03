use crate::descriptions::PixelFormat;
use crate::device_context::DeviceContext;
use crate::enums::{AlphaMode, BitmapOptions};
use crate::image::Bitmap1;
use crate::properties::BitmapProperties1;

use checked_enum::UncheckedEnum;
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
        dxgi: &'a dyn dxgi::surface::ISurface,
    },
}

impl<'a> BitmapBuilder1<'a> {
    pub(super) fn new(context: &'a DeviceContext) -> Self {
        BitmapBuilder1 {
            context,
            source: None,
            properties: BitmapProperties1 {
                pixel_format: PixelFormat {
                    format: Format::Unknown.into(),
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
        let mut properties = self.properties;
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = match source {
                Source::Memory { size, data, stride } => (*self.context.get_raw()).CreateBitmap(
                    size.into(),
                    data.as_ptr() as _,
                    stride,
                    &properties.into(),
                    &mut ptr,
                ),
                Source::Dxgi { dxgi } => {
                    let ddesc = dxgi.desc();
                    if properties.pixel_format.format == Format::Unknown {
                        properties.pixel_format.format = ddesc.format;
                    }
                    if ddesc.format != properties.pixel_format.format {
                        eprintln!(
                            "WARNING: Format of bitmap does not match backing surface. \
                             Surface format: {:?}, Bitmap format: {:?}",
                            ddesc.format, properties.pixel_format.format,
                        );
                        return Err(Error::INVALIDARG);
                    }

                    (*self.context.get_raw()).CreateBitmapFromDxgiSurface(
                        dxgi.raw_surface(),
                        &properties.into(),
                        &mut ptr,
                    )
                }
            };

            Error::map_if(hr, || Bitmap1::from_raw(ptr))
        }
    }

    pub fn with_image_data(self, size: impl Into<Sizeu>, data: &'a [u8], stride: u32) -> Self {
        let size = size.into();
        let fmt = self
            .properties
            .pixel_format
            .format
            .as_enum()
            .expect("`format` must be a known format to use `with_image_data.");
        if fmt == Format::Unknown {
            panic!(
                "You must call `with_format` before `with_image_data`, \
                 and the format cannot be `Unknown`."
            );
        }
        let psize = fmt.pixel_size();
        if psize == 0 {
            panic!(
                "Bytes per pixel of `{:?}` is unknown or not defined straightforwardly. If \
                 you would like to specify image data with this format, you must validate the \
                 buffer is appropriately sized yourself and use `with_image_data_unchecked`.",
                fmt
            );
        }

        let byte_width = size
            .width
            .checked_mul(psize as u32)
            .expect("Integer overflow on image width");

        assert!(byte_width <= stride, "Stride is too small for image width");

        let read_length = (stride as usize)
            .checked_mul(size.height as usize)
            .expect("Integer overflow on image height");

        assert!(
            read_length <= data.len(),
            "Image size is too large for length of buffer"
        );

        unsafe { self.with_image_data_unchecked(size, data, stride) }
    }

    pub fn with_dxgi_surface(mut self, dxgi: &'a dxgi::surface::Surface) -> Self {
        self.source = Some(Source::Dxgi { dxgi });
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.properties.pixel_format.format = format.into();
        self
    }

    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.properties.pixel_format.alpha_mode = alpha_mode.into();
        self
    }

    pub fn with_dpi(mut self, dpi_x: f32, dpi_y: f32) -> Self {
        self.properties.dpi_x = dpi_x;
        self.properties.dpi_y = dpi_y;
        self
    }

    pub fn with_options(mut self, options: BitmapOptions) -> Self {
        self.properties.options = options;
        self
    }
}

impl<'a> BitmapBuilder1<'a> {
    pub unsafe fn with_image_data_unchecked(
        mut self,
        size: impl Into<Sizeu>,
        data: &'a [u8],
        stride: u32,
    ) -> Self {
        let size = size.into();
        self.source = Some(Source::Memory { size, data, stride });
        self
    }

    pub unsafe fn with_format_unchecked(mut self, format: UncheckedEnum<Format>) -> Self {
        self.properties.pixel_format.format = format;
        self
    }
}
