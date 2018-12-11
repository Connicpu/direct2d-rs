use crate::device_context::DeviceContext;
use crate::device_context::DeviceContextType;
use crate::enums::{AlphaMode, BitmapOptions};
use crate::error::D2DResult;
use crate::image::{GenericImage, Image};
use crate::render_target::RenderTarget;
use crate::render_target::RenderTargetType;

use std::ptr;

use com_wrapper::ComWrapper;
use dxgi::enums::Format;
use dxgi::surface::Surface as DxgiSurface;
use math2d::{Sizef, Sizeu};
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Bitmap, ID2D1Image, D2D1_BITMAP_PROPERTIES};
use winapi::um::d2d1_1::D2D1_BITMAP_PROPERTIES1;
use winapi::um::dcommon::D2D1_PIXEL_FORMAT;
use wio::com::ComPtr;

#[derive(Clone)]
pub struct Bitmap {
    ptr: ComPtr<ID2D1Bitmap>,
}

impl Bitmap {
    #[inline]
    pub fn create<'a, R: RenderTargetType>(context: &'a R) -> BitmapBuilder<'a, R> {
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
    pub fn size(&self) -> Sizef {
        unsafe { self.ptr.GetSize().into() }
    }

    #[inline]
    pub fn pixel_size(&self) -> Sizeu {
        unsafe { self.ptr.GetPixelSize().into() }
    }

    #[inline]
    pub fn dpi(&self) -> (f32, f32) {
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

pub struct BitmapBuilder<'a, R: RenderTargetType> {
    context: &'a R,
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

impl<'a, R> BitmapBuilder<'a, R>
where
    R: RenderTargetType,
{
    #[inline]
    pub fn new(context: &'a R) -> Self {
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
        unsafe {
            match source {
                BitmapSource::Raw {
                    size,
                    source,
                    pitch,
                } => {
                    if self.properties.pixelFormat.format == Format::Unknown as u32 {
                        panic!("Format must be specified for use with raw data");
                    }

                    let source = source.map(<[_]>::as_ptr).unwrap_or(ptr::null());

                    let mut ptr = ptr::null_mut();

                    let mut hr = 0;
                    if self.properties.bitmapOptions == 0 && self.properties.colorContext.is_null()
                    {
                        let res = self.context.try_with_cast(|ctx: &RenderTarget| {
                            let ctx = &*ctx.get_raw();
                            hr = ctx.CreateBitmap(
                                size.into(),
                                source as *const _,
                                pitch,
                                properties as *const _ as *const D2D1_BITMAP_PROPERTIES,
                                &mut ptr,
                            );
                        });
                        assert!(res.is_some());
                    } else {
                        let res = self.context.try_with_cast(|ctx: &DeviceContext| {
                            let ctx = &*ctx.get_raw();
                            let mut ptr2 = ptr::null_mut();
                            hr = ctx.CreateBitmap(
                                size.into(),
                                source as *const _,
                                pitch,
                                properties,
                                &mut ptr2,
                            );
                            ptr = ptr2 as *mut _;
                        });
                        assert!(res.is_some());
                    }

                    if SUCCEEDED(hr) {
                        Ok(Bitmap::from_raw(ptr as _))
                    } else {
                        Err(hr.into())
                    }
                }
                BitmapSource::Dxgi(surface) => {
                    self.context.try_with_cast(|context: &DeviceContext| {
                        let mut ptr = std::ptr::null_mut();
                        let hr = (*context.get_raw()).CreateBitmapFromDxgiSurface(
                            surface.get_raw(),
                            properties,
                            &mut ptr,
                        );

                        if SUCCEEDED(hr) {
                            Ok(Bitmap::from_raw(ptr as _))
                        } else {
                            Err(hr.into())
                        }
                    }).unwrap()
                }
            }
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

impl<'a, D: DeviceContextType> BitmapBuilder<'a, D> {
    #[inline]
    pub fn with_dxgi_surface(mut self, surface: &'a DxgiSurface) -> Self {
        self.source = Some(BitmapSource::Dxgi(surface));
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
        size: Sizeu,
        source: Option<&'a [u8]>,
        pitch: u32,
    },
    Dxgi(&'a DxgiSurface),
}
