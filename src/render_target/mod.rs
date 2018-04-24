use brush::Brush;
use directwrite::{TextFormat, TextLayout};
use enums::DrawTextOptions;
use error::Error;
use factory::Factory;
use geometry::Geometry;
use math::*;
use std::{mem, ptr};
use stroke_style::StrokeStyle;
use wio::com::ComPtr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_TAG, ID2D1Factory, ID2D1RenderTarget};
use winapi::um::d2d1_1::ID2D1Factory1;
use winapi::um::dcommon::DWRITE_MEASURING_MODE_NATURAL;
use wio::wide::ToWide;

#[doc(inline)]
pub use self::dxgi::DxgiSurfaceRenderTarget;
#[doc(inline)]
pub use self::generic::GenericRenderTarget;
#[doc(inline)]
pub use self::hwnd::HwndRenderTarget;

pub mod dxgi;
pub mod generic;
pub mod hwnd;

#[derive(Copy, Clone, Debug)]
pub struct RenderTag {
    pub loc: &'static str,
}

#[repr(C)]
struct RenderTagRaw(usize, usize);

#[macro_export]
#[doc(hidden)]
macro_rules! make_render_tag {
    () => {
        $crate::render_target::RenderTag {
            loc: concat!(file!(), ':', line!()),
        }
    };
}

#[macro_export]
/// Use this to set a checkpoint that will be returned if flush() or end_draw() returns an
/// error to help debug which part of the drawing code is causing the error.
///
/// ```
/// # #[macro_use] extern crate direct2d;
/// # extern crate direct3d11;
/// # extern crate dxgi;
/// # use direct2d::{DeviceContext, RenderTarget};
/// # use direct2d::brush::SolidColorBrush;
/// # use direct2d::image::Bitmap;
/// fn draw(context: &mut DeviceContext, target: &Bitmap) {
///     let brush = SolidColorBrush::create(&context)
///         .with_color(0xFF7F7F)
///         .build().unwrap();
/// 
///     context.begin_draw();
///     context.set_target(target);
///
///     // Not sure which of these two lines could mess it up, so I set
///     // the render tag to be notified of the failure in the Err value.
///     set_render_tag!(context);
///     context.draw_line((10.0, 10.0), (20.0, 20.0), &brush, 2.0, None);
///
///     set_render_tag!(context);
///     context.draw_line((10.0, 20.0), (20.0, 10.0), &brush, 2.0, None);
///
///     match context.end_draw() {
///         Ok(_) => {/* cool */},
///         Err((err, Some(tag))) => {
///             panic!("Uh oh, rendering failed at {}: {}", tag.loc, err);
///         }
///         Err((err, None)) => {
///             panic!("Uh oh, rendering failed at an unknown location: {}", err);
///         }
///     }
/// }
/// # fn main() {
/// #     use direct2d::{Device, Factory};
/// #     use direct2d::enums::BitmapOptions;
/// #     use direct3d11::flags::{BindFlags, CreateDeviceFlags};
/// #     use dxgi::Format;
/// #     let (_, d3d, _) = direct3d11::device::Device::create()
/// #         .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
/// #         .build()
/// #         .unwrap();
/// #     let tex = direct3d11::texture2d::Texture2D::create(&d3d)
/// #         .with_size(64, 64)
/// #         .with_format(Format::R8G8B8A8Unorm)
/// #         .with_bind_flags(BindFlags::RENDER_TARGET | BindFlags::SHADER_RESOURCE)
/// #         .build()
/// #         .unwrap();
/// #     let factory = Factory::new().unwrap();
/// #     let dev = Device::create(&factory, &d3d.as_dxgi()).unwrap();
/// #     let mut ctx = DeviceContext::create(&dev, false).unwrap();
/// #     let target = Bitmap::create(&ctx)
/// #         .with_dxgi_surface(&tex.as_dxgi())
/// #         .with_dpi(192.0, 192.0)
/// #         .with_options(BitmapOptions::TARGET)
/// #         .build()
/// #         .unwrap();
/// #     draw(&mut ctx, &target);
/// # }
/// ```
macro_rules! set_render_tag {
    ($rt:ident) => {
        $crate::render_target::RenderTarget::set_tag($rt, make_render_tag!());
    };
}

impl<'r, R> RenderTarget for &'r mut R
where
    R: RenderTarget + 'r,
{
    #[doc(hidden)]
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        R::rt(*self)
    }
}

pub trait RenderTarget {
    #[doc(hidden)]
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget;

    #[doc(hidden)]
    unsafe fn make_tag(tag1: D2D1_TAG, tag2: D2D1_TAG) -> Option<RenderTag> {
        if tag1 == 0 {
            None
        } else {
            let raw = RenderTagRaw(tag1 as usize, tag2 as usize);
            let tag = mem::transmute(raw);
            Some(tag)
        }
    }

    fn get_factory(&mut self) -> Factory {
        unsafe {
            let mut ptr: *mut ID2D1Factory = ptr::null_mut();
            self.rt().GetFactory(&mut ptr);

            let ptr: ComPtr<ID2D1Factory1> = ComPtr::from_raw(ptr).cast().unwrap();
            Factory::from_raw(ptr.into_raw())
        }
    }

    fn get_size(&self) -> SizeF {
        unsafe { SizeF(self.rt().GetSize()) }
    }

    fn begin_draw(&mut self) {
        unsafe {
            self.rt().BeginDraw();
        }
    }

    fn end_draw(&mut self) -> Result<(), (Error, Option<RenderTag>)> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            let result = self.rt().EndDraw(&mut tag1, &mut tag2);

            if SUCCEEDED(result) {
                Ok(())
            } else {
                let tag = Self::make_tag(tag1, tag2);
                Err((From::from(result), tag))
            }
        }
    }

    fn set_tag(&mut self, tag: RenderTag) {
        unsafe {
            let RenderTagRaw(tag1, tag2) = mem::transmute(tag);
            self.rt().SetTags(tag1 as u64, tag2 as u64)
        };
    }

    fn get_tag(&mut self) -> Option<RenderTag> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            self.rt().GetTags(&mut tag1, &mut tag2);
            Self::make_tag(tag1, tag2)
        }
    }

    fn flush(&mut self) -> Result<(), (Error, Option<RenderTag>)> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            let result = self.rt().Flush(&mut tag1, &mut tag2);

            if SUCCEEDED(result) {
                Ok(())
            } else {
                let tag = Self::make_tag(tag1, tag2);
                Err((From::from(result), tag))
            }
        }
    }

    fn clear(&mut self, color: &ColorF) {
        unsafe {
            self.rt().Clear(&color.0);
        }
    }

    fn draw_line<P0, P1, B>(
        &mut self,
        p0: P0,
        p1: P1,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) where
        P0: Into<Point2F>,
        P1: Into<Point2F>,
        B: Brush,
    {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt().DrawLine(
                p0.into().0,
                p1.into().0,
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            )
        }
    }

    fn draw_rectangle<R, B>(
        &mut self,
        rect: R,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) where
        R: Into<RectF>,
        B: Brush,
    {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt()
                .DrawRectangle(&rect.into().0, brush.get_ptr(), stroke_width, stroke_style);
        }
    }

    fn fill_rectangle<R, B>(&mut self, rect: R, brush: &B)
    where
        R: Into<RectF>,
        B: Brush,
    {
        unsafe {
            self.rt().FillRectangle(&rect.into().0, brush.get_ptr());
        }
    }

    fn draw_rounded_rectangle<R, B>(
        &mut self,
        rect: R,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) where
        R: Into<RoundedRect>,
        B: Brush,
    {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt().DrawRoundedRectangle(
                &rect.into().0,
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            );
        }
    }

    fn fill_rounded_rectangle<R, B>(&mut self, rect: R, brush: &B)
    where
        R: Into<RoundedRect>,
        B: Brush,
    {
        unsafe {
            self.rt()
                .FillRoundedRectangle(&rect.into().0, brush.get_ptr());
        }
    }

    fn draw_ellipse<E, B>(
        &mut self,
        ellipse: E,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) where
        E: Into<Ellipse>,
        B: Brush,
    {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt().DrawEllipse(
                &ellipse.into().0,
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            );
        }
    }

    fn fill_ellipse<E, B>(&mut self, ellipse: E, brush: &B)
    where
        E: Into<Ellipse>,
        B: Brush,
    {
        unsafe {
            self.rt().FillEllipse(&ellipse.into().0, brush.get_ptr());
        }
    }

    fn draw_geometry<G: Geometry, B: Brush>(
        &mut self,
        geometry: &G,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt().DrawGeometry(
                geometry.get_ptr(),
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            );
        }
    }

    fn fill_geometry<G: Geometry, B: Brush>(&mut self, geometry: &G, brush: &B) {
        unsafe {
            self.rt()
                .FillGeometry(geometry.get_ptr(), brush.get_ptr(), ptr::null_mut());
        }
    }

    fn fill_geometry_with_opacity<G: Geometry, B: Brush, OB: Brush>(
        &mut self,
        geometry: &G,
        brush: &B,
        opacity_brush: &OB,
    ) {
        unsafe {
            self.rt()
                .FillGeometry(geometry.get_ptr(), brush.get_ptr(), opacity_brush.get_ptr());
        }
    }

    fn draw_text<B, R>(
        &mut self,
        text: &str,
        format: &TextFormat,
        layout_rect: R,
        foreground_brush: &B,
        options: DrawTextOptions,
    ) where
        R: Into<RectF>,
        B: Brush,
    {
        let text = text.to_wide_null();

        unsafe {
            let format = format.get_raw();
            self.rt().DrawText(
                text.as_ptr(),
                text.len() as u32,
                format,
                &layout_rect.into().0,
                foreground_brush.get_ptr(),
                options.0,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    fn draw_text_layout<P, B>(
        &mut self,
        origin: P,
        layout: &TextLayout,
        brush: &B,
        options: DrawTextOptions,
    ) where
        P: Into<Point2F>,
        B: Brush,
    {
        unsafe {
            let layout = layout.get_raw();
            self.rt()
                .DrawTextLayout(origin.into().0, layout, brush.get_ptr(), options.0);
        }
    }

    fn set_transform(&mut self, transform: &Matrix3x2F) {
        unsafe { self.rt().SetTransform(&transform.0) }
    }

    fn get_transform(&self) -> Matrix3x2F {
        unsafe {
            let mut mat: Matrix3x2F = mem::uninitialized();
            self.rt().GetTransform(&mut mat.0);
            mat
        }
    }
}
