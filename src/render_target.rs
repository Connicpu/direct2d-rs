use std::{ptr, mem};
use winapi::*;
use math::*;
use brush::Brush;
use error::D2D1Error;
use stroke_style::StrokeStyle;
use comptr::ComPtr;
use helpers::{GetRaw, FromRaw};

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

struct RenderTagRaw(D2D1_TAG, D2D1_TAG);

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
        $rt.set_tag(make_render_tag!())
    }
}

impl RenderTarget {
    unsafe fn rt<'a>(&self) -> &'a mut ID2D1RenderTarget {
        &mut *self.ptr.raw_value()
    }
    
    unsafe fn make_tag(tag1: D2D1_TAG, tag2: D2D1_TAG) -> Option<RenderTag> {
        if tag1 == 0 {
            None
        } else {
            let raw = RenderTagRaw(tag1, tag2);
            let tag = mem::transmute(raw);
            Some(tag)
        }
    }
    
    pub fn set_tag(&mut self, tag: RenderTag) {
        unsafe {
            let RenderTagRaw(tag1, tag2) = mem::transmute(tag);
            self.rt().SetTags(tag1, tag2)
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
    
    pub fn draw_line<B: Brush>(
        &mut self, p0: Point2F, p1: Point2F, brush: &B, stroke_width: f32,
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
