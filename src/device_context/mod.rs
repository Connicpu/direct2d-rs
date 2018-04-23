use image::Bitmap;
use error::Error;
use helpers::{ret_obj, FromRaw, GetRaw};
use image::Image;
use render_target::RenderTarget;

use std::ptr;

use dxgi::surface::Surface;
use winapi::um::d2d1::ID2D1RenderTarget;
use winapi::um::d2d1_1::{D2D1_BITMAP_OPTIONS_TARGET, D2D1_BITMAP_PROPERTIES1, ID2D1DeviceContext};
use winapi::um::dcommon::{D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_PIXEL_FORMAT};
use wio::com::ComPtr;

pub struct DeviceContext {
    ptr: ComPtr<ID2D1DeviceContext>,
}

impl DeviceContext {
    pub fn create_bitmap_from_dxgi_surface(
        &self,
        dpi: (f32, f32),
        target: bool,
        surface: &Surface,
    ) -> Result<Bitmap, Error> {
        let surf_desc = surface.get_desc();
        unsafe {
            let props = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: surf_desc.format(),
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: dpi.0,
                dpiY: dpi.1,
                bitmapOptions: if target {
                    D2D1_BITMAP_OPTIONS_TARGET
                } else {
                    0
                },
                colorContext: ptr::null_mut(),
            };

            let mut ptr = ptr::null_mut();
            let hr = self.ptr
                .CreateBitmapFromDxgiSurface(surface.get_raw(), &props, &mut ptr);

            ret_obj(hr, ptr as *mut _)
        }
    }

    pub fn set_target<I>(&mut self, target: &I)
    where
        I: Image,
    {
        unsafe {
            self.ptr.SetTarget(target.get_ptr());
        }
    }
}

impl RenderTarget for DeviceContext {
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *(self.ptr.as_raw() as *mut _)
    }
}

unsafe impl Send for DeviceContext {}
unsafe impl Sync for DeviceContext {}

impl FromRaw for DeviceContext {
    type Raw = ID2D1DeviceContext;
    unsafe fn from_raw(raw: *mut ID2D1DeviceContext) -> Self {
        DeviceContext {
            ptr: ComPtr::from_raw(raw),
        }
    }
}

impl GetRaw for DeviceContext {
    type Raw = ID2D1DeviceContext;
    unsafe fn get_raw(&self) -> *mut ID2D1DeviceContext {
        self.ptr.as_raw()
    }
}
