use std::{mem, ptr};
use math::*;
use brush::{self, Brush, ExtendMode, GradientStop, GradientStopCollection, LinearGradientBrush};
use geometry::Geometry;
use error::Error;
use stroke_style::StrokeStyle;
use factory::Factory;
use wio::com::ComPtr;
use helpers::{ret_obj, FromRaw, GetRaw};
use directwrite::{TextFormat, TextLayout};

use winapi::shared::winerror::{HRESULT, SUCCEEDED};
use winapi::um::d2d1::{D2D1_DRAW_TEXT_OPTIONS_NONE, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
                       D2D1_TAG, ID2D1Factory, ID2D1HwndRenderTarget, ID2D1RenderTarget,
                       ID2D1SolidColorBrush};
use winapi::um::d2d1_1::ID2D1Factory1;
use winapi::um::dcommon::DWRITE_MEASURING_MODE_NATURAL;
use wio::wide::ToWide;

/// This trait is intended to be implemented by external APIs who would
/// like to allow a Direct2D interface for drawing onto them. Since the
/// Direct2D creation APIs require more information, this will likely be
/// implemented on Builder objects which contain enough context about
/// the actual backing structure and what needs to be done to set it up.
pub unsafe trait RenderTargetBacking {
    /// The ID2D1RenderTarget's ownership is passed out of the function and as such
    /// the caller is now responsible for ensuring the pointer will receive its
    /// Release() call.
    fn create_target(self, factory: &mut ID2D1Factory1) -> Result<*mut ID2D1RenderTarget, HRESULT>;
}

pub struct ConcreteRenderTarget {
    ptr: ComPtr<ID2D1RenderTarget>,
}

unsafe impl Send for ConcreteRenderTarget {}
unsafe impl Sync for ConcreteRenderTarget {}

#[derive(Copy, Clone, Debug)]
pub struct RenderTag {
    pub file_line: &'static str,
}

struct RenderTagRaw(usize, usize);

#[macro_export]
macro_rules! make_render_tag {
    () => {
        RenderTag {
            file: concat!(file!(), ':', line!())
        }
    }
}

#[macro_export]
macro_rules! set_render_tag {
    ($rt:ident) => {
        $crate::render_target::RenderTarget::set_tag(&mut $rt, make_render_tag!());
    }
}

impl ConcreteRenderTarget {
    pub unsafe fn as_hwnd_rt(&self) -> Option<ComPtr<ID2D1HwndRenderTarget>> {
        self.ptr.cast().ok()
    }
}

impl RenderTarget for ConcreteRenderTarget {
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *self.ptr.as_raw()
    }
}

impl GetRaw for ConcreteRenderTarget {
    type Raw = ID2D1RenderTarget;
    unsafe fn get_raw(&self) -> *mut ID2D1RenderTarget {
        self.ptr.as_raw()
    }
}

impl FromRaw for ConcreteRenderTarget {
    type Raw = ID2D1RenderTarget;
    unsafe fn from_raw(raw: *mut ID2D1RenderTarget) -> Self {
        ConcreteRenderTarget {
            ptr: ComPtr::from_raw(raw),
        }
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
            let mut factory: *mut ID2D1Factory = ptr::null_mut();
            self.rt().GetFactory(&mut factory);

            Factory::from_ptr(ComPtr::from_raw(factory).cast().unwrap())
        }
    }

    fn get_size(&self) -> SizeF {
        unsafe { SizeF(self.rt().GetSize()) }
    }

    fn create_solid_color_brush<C: Into<ColorF>>(
        &self,
        color: C,
        props: &BrushProperties,
    ) -> Result<brush::SolidColor, Error> {
        unsafe {
            let mut ptr: *mut ID2D1SolidColorBrush = ptr::null_mut();
            let hr = self.rt()
                .CreateSolidColorBrush(&color.into().0, &props.0, &mut ptr);

            ret_obj(hr, ptr)
        }
    }

    fn create_gradient_stop_collection(
        &self,
        stops: &[GradientStop],
        extend: ExtendMode,
    ) -> Result<GradientStopCollection, Error> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.rt().CreateGradientStopCollection(
                stops.as_ptr() as *const _,
                stops.len() as u32,
                0,
                extend as u32,
                &mut ptr,
            );

            ret_obj(hr, ptr)
        }
    }

    fn create_linear_gradient_brush(
        &self,
        start: Point2F,
        end: Point2F,
        props: &BrushProperties,
        stops: &GradientStopCollection,
    ) -> Result<LinearGradientBrush, Error> {
        unsafe {
            let lin_props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                startPoint: start.0,
                endPoint: end.0,
            };

            let mut ptr = ptr::null_mut();
            let hr = self.rt().CreateLinearGradientBrush(
                &lin_props,
                &props.0,
                stops.get_raw(),
                &mut ptr,
            );

            ret_obj(hr, ptr)
        }
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
                let tag = ConcreteRenderTarget::make_tag(tag1, tag2);
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
            ConcreteRenderTarget::make_tag(tag1, tag2)
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
                let tag = ConcreteRenderTarget::make_tag(tag1, tag2);
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
        options: &[DrawTextOption],
    ) {
        let text = text.to_wide_null();
        let mut draw_options = D2D1_DRAW_TEXT_OPTIONS_NONE;
        for &option in options {
            draw_options |= option as u32;
        }

        unsafe {
            let format = format.get_raw();
            self.rt().DrawText(
                text.as_ptr(),
                text.len() as u32,
                format,
                &layout_rect.0,
                foreground_brush.get_ptr(),
                draw_options,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    fn draw_text_layout<B: Brush>(
        &mut self,
        origin: &Point2F,
        layout: &TextLayout,
        brush: &B,
        options: &[DrawTextOption],
    ) {
        let mut draw_options = D2D1_DRAW_TEXT_OPTIONS_NONE;
        for &option in options {
            draw_options |= option as u32;
        }

        unsafe {
            let layout = layout.get_raw();
            self.rt()
                .DrawTextLayout(origin.0, layout, brush.get_ptr(), draw_options);
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawTextOption {
    NoSnap = 1,
    Clip = 2,
    EnableColorFont = 4,
}
