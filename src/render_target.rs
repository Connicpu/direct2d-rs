use std::{ptr, mem};
use winapi::*;
use math::*;
use brush::{self, Brush};
use geometry::{Geometry};
use error::D2D1Error;
use stroke_style::StrokeStyle;
use factory::Factory;
use comptr::ComPtr;
use helpers::{GetRaw, FromRaw, ToWide};
use directwrite::{TextFormat, TextLayout};

/// This trait is intended to be implemented by external APIs who would
/// like to allow a Direct2D interface for drawing onto them. Since the
/// Direct2D creation APIs require more information, this will likely be
/// implemented on Builder objects which contain enough context about
/// the actual backing structure and what needs to be done to set it up.
pub unsafe trait RenderTargetBacking {
    /// The ID2D1RenderTarget's ownership is passed out of the function and as such
    /// the caller is now responsible for ensuring the pointer will receive its
    /// Release() call.
    fn create_target(self, factory: &mut ID2D1Factory) -> Result<*mut ID2D1RenderTarget, HRESULT>;
}

#[derive(Clone, Debug)]
pub struct RenderTarget {
    ptr: ComPtr<ID2D1RenderTarget>,
}

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
        $rt.set_tag(make_render_tag!());
    }
}

impl RenderTarget {
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *self.ptr.raw_value()
    }
    
    pub unsafe fn hwnd_rt(&self) -> Option<ComPtr<ID2D1HwndRenderTarget>> {
        self.ptr.query_interface().ok()
    }
    
    unsafe fn make_tag(tag1: D2D1_TAG, tag2: D2D1_TAG) -> Option<RenderTag> {
        if tag1 == 0 {
            None
        } else {
            let raw = RenderTagRaw(tag1 as usize, tag2 as usize);
            let tag = mem::transmute(raw);
            Some(tag)
        }
    }
    
    pub fn get_factory(&mut self) -> Factory {
        unsafe {
            let mut factory = ComPtr::<ID2D1Factory>::new();
            self.rt().GetFactory(factory.raw_addr());
            
            Factory::from_ptr(factory)
        }
    }
    
    pub fn get_size(&self) -> SizeF {
        unsafe {
            let mut size = mem::uninitialized();
            self.rt().GetSize(&mut size);
            SizeF(size)
        }
    }
    
    pub fn create_solid_color_brush<C: Into<ColorF>>(
        &self, color: C, props: &BrushProperties
    ) -> Result<brush::SolidColor, D2D1Error> {
        unsafe {
            let mut ptr = ComPtr::<ID2D1SolidColorBrush>::new();
            let result = self.rt().CreateSolidColorBrush(&color.into().0, &props.0, ptr.raw_addr());
            
            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr.raw_value()))
            } else {
                Err(From::from(result))
            }
        }
    }
    
    pub fn begin_draw(&mut self) {
        unsafe {
            self.rt().BeginDraw();
        }
    }
    
    pub fn end_draw(&mut self) -> Result<(), (D2D1Error, Option<RenderTag>)> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            let result = self.rt().EndDraw(&mut tag1, &mut tag2);
            
            if SUCCEEDED(result) {
                Ok(())
            } else {
                let tag = RenderTarget::make_tag(tag1, tag2);
                Err((From::from(result), tag))
            }
        }
    }
    
    pub fn set_tag(&mut self, tag: RenderTag) {
        unsafe {
            let RenderTagRaw(tag1, tag2) = mem::transmute(tag);
            self.rt().SetTags(tag1 as u64, tag2 as u64)
        };
    }
    
    pub fn get_tag(&mut self) -> Option<RenderTag> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            self.rt().GetTags(&mut tag1, &mut tag2);
            RenderTarget::make_tag(tag1, tag2)
        }
    }
    
    pub fn flush(&mut self) -> Result<(), (D2D1Error, Option<RenderTag>)> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            let result = self.rt().Flush(&mut tag1, &mut tag2);
            
            if SUCCEEDED(result) {
                Ok(())
            } else {
                let tag = RenderTarget::make_tag(tag1, tag2);
                Err((From::from(result), tag))
            }
        }
    }
    
    pub fn clear(&mut self, color: &ColorF) {
        unsafe {
            self.rt().Clear(&color.0);
        }
    }
    
    pub fn draw_line<B: Brush>(
        &mut self, p0: &Point2F, p1: &Point2F, brush: &B, stroke_width: f32,
        stroke_style: Option<&StrokeStyle>
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr(),
                None => ptr::null_mut(),
            };
            
            self.rt().DrawLine(
                p0.0,
                p1.0,
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            )
        }
    }
    
    pub fn draw_rectangle<B: Brush>(
        &mut self, rect: &RectF, brush: &B, stroke_width: f32, stroke_style: Option<&StrokeStyle>
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr(),
                None => ptr::null_mut(),
            };
            
            self.rt().DrawRectangle(
                &rect.0,
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            );
        }
    }
    
    pub fn fill_rectangle<B: Brush>(&mut self, rect: &RectF, brush: &B) {
        unsafe {
            self.rt().FillRectangle(
                &rect.0,
                brush.get_ptr(),
            );
        }
    }
    
    pub fn draw_rounded_rectangle<B: Brush>(
        &mut self, rect: &RoundedRect, brush: &B, stroke_width: f32, stroke_style: Option<&StrokeStyle>
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr(),
                None => ptr::null_mut(),
            };
            
            self.rt().DrawRoundedRectangle(
                &rect.0,
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            );
        }
    }
    
    pub fn fill_rounded_rectangle<B: Brush>(&mut self, rect: &RoundedRect, brush: &B) {
        unsafe {
            self.rt().FillRoundedRectangle(
                &rect.0,
                brush.get_ptr(),
            );
        }
    }
    
    pub fn draw_ellipse<B: Brush>(
        &mut self, ellipse: &Ellipse, brush: &B, stroke_width: f32, stroke_style: Option<&StrokeStyle>
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr(),
                None => ptr::null_mut(),
            };
            
            self.rt().DrawEllipse(
                &ellipse.0,
                brush.get_ptr(),
                stroke_width,
                stroke_style,
            );
        }
    }
    
    pub fn fill_ellipse<B: Brush>(&mut self, ellipse: &Ellipse, brush: &B) {
        unsafe {
            self.rt().FillEllipse(
                &ellipse.0,
                brush.get_ptr(),
            );
        }
    }
    
    pub fn draw_geometry<G: Geometry, B: Brush>(
        &mut self, geometry: &G, brush: &B, stroke_width: f32, stroke_style: Option<&StrokeStyle>
    ) {
        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_ptr(),
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
    
    pub fn fill_geometry<G: Geometry, B: Brush>(&mut self, geometry: &G, brush: &B) {
        unsafe {
            self.rt().FillGeometry(
                geometry.get_ptr(),
                brush.get_ptr(),
                ptr::null_mut(),
            );
        }
    }
    
    pub fn fill_geometry_with_opacity<G: Geometry, B: Brush, OB: Brush>(
        &mut self, geometry: &G, brush: &B, opacity_brush: &OB
    ) {
        unsafe {
            self.rt().FillGeometry(
                geometry.get_ptr(),
                brush.get_ptr(),
                opacity_brush.get_ptr(),
            );
        }
    }
    
    pub fn draw_text<B: Brush>(
        &mut self, text: &str, format: &TextFormat, layout_rect: &RectF, foreground_brush: &B,
        options: &[DrawTextOption],
    ) {
        let text = text.to_wide_null();
        let mut draw_options = D2D1_DRAW_TEXT_OPTIONS_NONE.0;
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
                D2D1_DRAW_TEXT_OPTIONS(draw_options),
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }
    
    pub fn draw_text_layout<B: Brush>(
        &mut self, origin: &Point2F, layout: &TextLayout, brush: &B, options: &[DrawTextOption],
    ) {
        let mut draw_options = D2D1_DRAW_TEXT_OPTIONS_NONE.0;
        for &option in options {
            draw_options |= option as u32;
        }
        
        unsafe {
            let layout = layout.get_raw();
            self.rt().DrawTextLayout(
                origin.0,
                layout,
                brush.get_ptr(),
                D2D1_DRAW_TEXT_OPTIONS(draw_options),
            );
        }
    }
}

impl GetRaw for RenderTarget {
    type Raw = ID2D1RenderTarget;
    unsafe fn get_raw(&self) -> *mut ID2D1RenderTarget {
        self.ptr.raw_value()
    }
}

impl FromRaw for RenderTarget {
    type Raw = ID2D1RenderTarget;
    unsafe fn from_raw(raw: *mut ID2D1RenderTarget) -> Self {
        RenderTarget {
            ptr: ComPtr::attach(raw)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawTextOption {
    NoSnap = 1,
    Clip = 2,
    EnableColorFont = 4,
}
