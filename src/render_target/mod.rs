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

pub mod hwnd;

#[derive(Copy, Clone, Debug)]
pub struct RenderTag {
    pub file_line: &'static str,
}

#[repr(C)]
struct RenderTagRaw(usize, usize);

#[macro_export]
macro_rules! make_render_tag {
    () => {
        RenderTag {
            file: concat!(file!(), ':', line!()),
        }
    };
}

#[macro_export]
macro_rules! set_render_tag {
    ($rt:ident) => {
        $crate::render_target::RenderTarget::set_tag(&mut $rt, make_render_tag!());
    };
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

    fn draw_line<B: Brush>(
        &mut self,
        p0: &Point2F,
        p1: &Point2F,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt()
                .DrawLine(p0.0, p1.0, brush.get_ptr(), stroke_width, stroke_style)
        }
    }

    fn draw_rectangle<B: Brush>(
        &mut self,
        rect: &RectF,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt()
                .DrawRectangle(&rect.0, brush.get_ptr(), stroke_width, stroke_style);
        }
    }

    fn fill_rectangle<B: Brush>(&mut self, rect: &RectF, brush: &B) {
        unsafe {
            self.rt().FillRectangle(&rect.0, brush.get_ptr());
        }
    }

    fn draw_rounded_rectangle<B: Brush>(
        &mut self,
        rect: &RoundedRect,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt()
                .DrawRoundedRectangle(&rect.0, brush.get_ptr(), stroke_width, stroke_style);
        }
    }

    fn fill_rounded_rectangle<B: Brush>(&mut self, rect: &RoundedRect, brush: &B) {
        unsafe {
            self.rt().FillRoundedRectangle(&rect.0, brush.get_ptr());
        }
    }

    fn draw_ellipse<B: Brush>(
        &mut self,
        ellipse: &Ellipse,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr() as *mut _,
                None => ptr::null_mut(),
            };

            self.rt()
                .DrawEllipse(&ellipse.0, brush.get_ptr(), stroke_width, stroke_style);
        }
    }

    fn fill_ellipse<B: Brush>(&mut self, ellipse: &Ellipse, brush: &B) {
        unsafe {
            self.rt().FillEllipse(&ellipse.0, brush.get_ptr());
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
                Some(s) => s.get_ptr() as *mut _,
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

    fn draw_text<B: Brush>(
        &mut self,
        text: &str,
        format: &TextFormat,
        layout_rect: &RectF,
        foreground_brush: &B,
        options: DrawTextOptions,
    ) {
        let text = text.to_wide_null();

        unsafe {
            let format = format.get_raw();
            self.rt().DrawText(
                text.as_ptr(),
                text.len() as u32,
                format,
                &layout_rect.0,
                foreground_brush.get_ptr(),
                options.0,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    fn draw_text_layout<B: Brush>(
        &mut self,
        origin: &Point2F,
        layout: &TextLayout,
        brush: &B,
        options: DrawTextOptions,
    ) {
        unsafe {
            let layout = layout.get_raw();
            self.rt()
                .DrawTextLayout(origin.0, layout, brush.get_ptr(), options.0);
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
