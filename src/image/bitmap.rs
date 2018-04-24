use device_context::DeviceContext;
use error::D2DResult;
use image::{GenericImage, Image};
use math::{SizeF, SizeU};

use std::ptr;

use dxgi::surface::Surface as DxgiSurface;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Bitmap, ID2D1Image};
use winapi::um::d2d1_1::{D2D1_BITMAP_OPTIONS_TARGET, D2D1_BITMAP_PROPERTIES1};
use winapi::um::dcommon::{D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_PIXEL_FORMAT};
use wio::com::ComPtr;

#[derive(Clone)]
pub struct Bitmap {
    ptr: ComPtr<ID2D1Bitmap>,
}

impl Bitmap {
    pub fn create_from_dxgi(
        context: &DeviceContext,
        surface: &DxgiSurface,
        dpi: (f32, f32),
        is_target: bool,
    ) -> D2DResult<Bitmap> {
        let surf_desc = surface.get_desc();
        unsafe {
            let props = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: surf_desc.format(),
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: dpi.0,
                dpiY: dpi.1,
                bitmapOptions: if is_target {
                    D2D1_BITMAP_OPTIONS_TARGET
                } else {
                    0
                },
                colorContext: ptr::null_mut(),
            };

            let mut ptr = ptr::null_mut();
            let hr = (*context.get_raw()).CreateBitmapFromDxgiSurface(
                surface.get_raw(),
                &props,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(Bitmap::from_raw(ptr as *mut _))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn as_generic(&self) -> GenericImage {
        unsafe {
            let ptr = self.get_raw();
            (*ptr).AddRef();
            GenericImage::from_raw(ptr as *mut _)
        }
    }

    pub fn get_size(&self) -> SizeF {
        unsafe { SizeF(self.ptr.GetSize()) }
    }

    pub fn get_pixel_size(&self) -> SizeU {
        unsafe { SizeU(self.ptr.GetPixelSize()) }
    }

    pub fn get_dpi(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            self.ptr.GetDpi(&mut x, &mut y);
        }
        (x, y)
    }

    pub unsafe fn get_raw(&self) -> *mut ID2D1Bitmap {
        self.ptr.as_raw()
    }

    pub unsafe fn from_raw(ptr: *mut ID2D1Bitmap) -> Self {
        Bitmap {
            ptr: ComPtr::from_raw(ptr),
        }
    }
}

impl Image for Bitmap {
    unsafe fn get_ptr(&self) -> *mut ID2D1Image {
        self.ptr.as_raw() as *mut _
    }
}

unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}
