use device_context::DeviceContext;
use enums::{AlphaMode, BitmapOptions};
use error::D2DResult;
use image::{GenericImage, Image};
use math::{SizeF, SizeU};

use std::ptr;

use dxgi::surface::Surface as DxgiSurface;
use dxgi::Format;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Bitmap, ID2D1Image};
use winapi::um::d2d1_1::D2D1_BITMAP_PROPERTIES1;
use winapi::um::dcommon::D2D1_PIXEL_FORMAT;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct Bitmap {
    ptr: ComPtr<ID2D1Bitmap>,
}

impl Bitmap {
    #[inline]
    pub fn create<'a>(context: &'a DeviceContext) -> BitmapBuilder<'a> {
        BitmapBuilder::new(context)
    }

    #[inline]
    pub fn as_generic(&self) -> GenericImage {
        unsafe {
            let ptr = self.get_raw();
            (*ptr).AddRef();
            GenericImage::from_raw(ptr as *mut _)
        }
    }

    #[inline]
    pub fn get_size(&self) -> SizeF {
        unsafe { SizeF(self.ptr.GetSize()) }
    }

    #[inline]
    pub fn get_pixel_size(&self) -> SizeU {
        unsafe { SizeU(self.ptr.GetPixelSize()) }
    }

    #[inline]
    pub fn get_dpi(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            self.ptr.GetDpi(&mut x, &mut y);
        }
        (x, y)
    }

    #[inline]
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1Bitmap>) -> Self {
        Self { ptr }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut ID2D1Bitmap {
        self.ptr.as_raw()
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut ID2D1Bitmap) -> Self {
        Bitmap {
            ptr: ComPtr::from_raw(ptr),
        }
    }
}

impl Image for Bitmap {
    #[inline]
    unsafe fn get_ptr(&self) -> *mut ID2D1Image {
        self.ptr.as_raw() as *mut _
    }
}

unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}

pub struct BitmapBuilder<'a> {
    context: &'a DeviceContext,
    source: Option<BitmapSource<'a>>,
    properties: D2D1_BITMAP_PROPERTIES1,
}

const DEFAULT_BITMAP_PROPS: D2D1_BITMAP_PROPERTIES1 = D2D1_BITMAP_PROPERTIES1 {
    pixelFormat: D2D1_PIXEL_FORMAT {
        format: Format::Unknown as u32,
        alphaMode: AlphaMode::Premultiplied as u32,
    },
    dpiX: 96.0,
    dpiY: 96.0,
    bitmapOptions: BitmapOptions::NONE.0,
    colorContext: ptr::null(),
};

impl<'a> BitmapBuilder<'a> {
    #[inline]
    pub fn new(context: &'a DeviceContext) -> Self {
        BitmapBuilder {
            context,
            source: None,
            properties: DEFAULT_BITMAP_PROPS,
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<Bitmap> {
        let source = self.source.expect("An image source must be specified");
        unsafe {
            match source {
                BitmapSource::Raw {
                    size,
                    source,
                    pitch,
                } => {
                    let source = source.map(<[_]>::as_ptr).unwrap_or(ptr::null());

                    let mut ptr = ptr::null_mut();
                    let hr = (*self.context.get_raw()).CreateBitmap(
                        size.0,
                        source as *const _,
                        pitch,
                        &self.properties,
                        &mut ptr,
                    );

                    if SUCCEEDED(hr) {
                        Ok(Bitmap::from_raw(ptr as _))
                    } else {
                        Err(hr.into())
                    }
                }
                BitmapSource::Dxgi(surface) => {
                    let mut ptr = ptr::null_mut();
                    let hr = (*self.context.get_raw()).CreateBitmapFromDxgiSurface(
                        surface.get_raw(),
                        &self.properties,
                        &mut ptr,
                    );

                    if SUCCEEDED(hr) {
                        Ok(Bitmap::from_raw(ptr as _))
                    } else {
                        Err(hr.into())
                    }
                }
            }
        }
    }

    #[inline]
    pub fn with_blank_image(mut self, size: SizeU) -> Self {
        self.source = Some(BitmapSource::Raw {
            size,
            source: None,
            pitch: 0,
        });
        self
    }

    #[inline]
    pub fn with_raw_data(mut self, size: SizeU, data: &'a [u8], pitch: u32) -> Self {
        assert!(size.height as usize * pitch as usize <= data.len());
        self.source = Some(BitmapSource::Raw {
            size,
            source: Some(data),
            pitch,
        });
        self
    }

    #[inline]
    pub fn with_dxgi_surface(mut self, surface: &'a DxgiSurface) -> Self {
        self.source = Some(BitmapSource::Dxgi(surface));
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
        println!("setting DPI to: {:?}", (dpi_x, dpi_y));
        self.properties.dpiX = dpi_x;
        self.properties.dpiY = dpi_y;
        self
    }

    #[inline]
    pub fn with_options(mut self, options: BitmapOptions) -> Self {
        self.properties.bitmapOptions = options.0;
        self
    }
}

enum BitmapSource<'a> {
    Raw {
        size: SizeU,
        source: Option<&'a [u8]>,
        pitch: u32,
    },
    Dxgi(&'a DxgiSurface),
}
