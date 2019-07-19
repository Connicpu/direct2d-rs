use crate::brush::Brush;
use crate::enums::{AntialiasMode, BitmapInterpolationMode, DrawTextOptions};
use crate::geometry::Geometry;
use crate::image::Bitmap;
use crate::layer::{Layer, LayerBuilder};
use crate::stroke_style::StrokeStyle;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::error::Error;
use directwrite::{TextFormat, TextLayout};
use math2d::*;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1RenderTarget, D2D1_TAG};
use winapi::um::dcommon::DWRITE_MEASURING_MODE_NATURAL;
use wio::com::ComPtr;
use wio::wide::ToWide;

pub use self::hwnd::HwndRenderTarget;
pub use self::render_tag::RenderTag;

pub mod hwnd;
pub mod render_tag;

#[repr(C)]
pub struct RenderTarget {
    ptr: ComPtr<ID2D1RenderTarget>,
    state: RTState,
}

impl RenderTarget {
    #[inline]
    pub fn size(&self) -> Sizef {
        unsafe { self.rt().GetSize().into() }
    }

    #[inline]
    pub fn pixel_size(&self) -> Sizeu {
        unsafe { self.rt().GetPixelSize().into() }
    }

    #[inline]
    pub fn begin_draw(&mut self) {
        if !self.state.is_set(RTState::NOT_DRAWING) {
            panic!("You may not call begin_draw() when you are already drawing.");
        }

        unsafe {
            self.rt().BeginDraw();
            self.state.clear(RTState::NOT_DRAWING);
        }
    }

    #[inline]
    pub fn end_draw(&mut self) -> Result<(), (Error, Option<RenderTag>)> {
        self.assert_can_draw("end_draw");

        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            let hr = self.rt().EndDraw(&mut tag1, &mut tag2);
            self.state |= RTState::NOT_DRAWING;

            if SUCCEEDED(hr) {
                Ok(())
            } else {
                let tag = Self::make_tag(tag1, tag2);
                Err((From::from(hr), tag))
            }
        }
    }

    #[inline]
    pub fn set_tag(&mut self, tag: Option<RenderTag>) {
        unsafe {
            if let Some(tag) = tag {
                let (tag1, tag2) = tag.to_raw();
                self.rt().SetTags(tag1 as u64, tag2 as u64)
            } else {
                self.rt().SetTags(0, 0);
            }
        };
    }

    #[inline]
    pub fn get_tag(&self) -> Option<RenderTag> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            self.rt().GetTags(&mut tag1, &mut tag2);
            Self::make_tag(tag1, tag2)
        }
    }

    #[inline]
    pub fn flush(&mut self) -> Result<(), (Error, Option<RenderTag>)> {
        self.assert_can_draw("flush");

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

    #[inline]
    pub fn clear(&mut self, color: impl Into<Color>) {
        self.assert_can_draw("clear");

        unsafe {
            self.rt().Clear(&color.into().into());
        }
    }

    #[inline]
    pub fn draw_line(
        &mut self,
        p0: impl Into<Point2f>,
        p1: impl Into<Point2f>,
        brush: &Brush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_line");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.rt().DrawLine(
                p0.into().into(),
                p1.into().into(),
                brush.get_raw(),
                stroke_width,
                stroke_style,
            )
        }
    }

    #[inline]
    pub fn draw_rectangle(
        &mut self,
        rect: impl Into<Rectf>,
        brush: &Brush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_rectangle");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.rt().DrawRectangle(
                &rect.into().into(),
                brush.get_raw(),
                stroke_width,
                stroke_style,
            );
        }
    }

    #[inline]
    pub fn fill_rectangle(&mut self, rect: impl Into<Rectf>, brush: &Brush) {
        self.assert_can_draw("fill_rectangle");

        unsafe {
            self.rt()
                .FillRectangle(&rect.into().into(), brush.get_raw());
        }
    }

    #[inline]
    pub fn draw_rounded_rectangle(
        &mut self,
        rect: impl Into<RoundedRect>,
        brush: &Brush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_rounded_rectangle");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.rt().DrawRoundedRectangle(
                &rect.into().into(),
                brush.get_raw(),
                stroke_width,
                stroke_style,
            );
        }
    }

    #[inline]
    pub fn fill_rounded_rectangle(&mut self, rect: impl Into<RoundedRect>, brush: &Brush) {
        self.assert_can_draw("fill_rounded_rectangle");

        unsafe {
            self.rt()
                .FillRoundedRectangle(&rect.into().into(), brush.get_raw());
        }
    }

    #[inline]
    pub fn draw_ellipse(
        &mut self,
        ellipse: impl Into<Ellipse>,
        brush: &Brush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_ellipse");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.rt().DrawEllipse(
                &ellipse.into().into(),
                brush.get_raw(),
                stroke_width,
                stroke_style,
            );
        }
    }

    #[inline]
    pub fn fill_ellipse(&mut self, ellipse: impl Into<Ellipse>, brush: &Brush) {
        self.assert_can_draw("fill_ellipse");

        unsafe {
            self.rt()
                .FillEllipse(&ellipse.into().into(), brush.get_raw());
        }
    }

    #[inline]
    pub fn draw_geometry(
        &mut self,
        geometry: &Geometry,
        brush: &Brush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_geometry");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.rt().DrawGeometry(
                geometry.get_raw(),
                brush.get_raw(),
                stroke_width,
                stroke_style,
            );
        }
    }

    #[inline]
    pub fn fill_geometry(&mut self, geometry: &Geometry, brush: &Brush) {
        self.assert_can_draw("fill_geometry");

        unsafe {
            self.rt()
                .FillGeometry(geometry.get_raw(), brush.get_raw(), std::ptr::null_mut());
        }
    }

    #[inline]
    pub fn fill_geometry_with_opacity(
        &mut self,
        geometry: &Geometry,
        brush: &Brush,
        opacity_brush: &Brush,
    ) {
        self.assert_can_draw("fill_geometry_with_opacity");

        unsafe {
            self.rt()
                .FillGeometry(geometry.get_raw(), brush.get_raw(), opacity_brush.get_raw());
        }
    }

    #[inline]
    pub fn draw_bitmap(
        &mut self,
        bitmap: &Bitmap,
        dest_rect: impl Into<Rectf>,
        opacity: f32,
        interpolation: BitmapInterpolationMode,
        src_rect: impl Into<Rectf>,
    ) {
        self.assert_can_draw("draw_bitmap");

        unsafe {
            self.rt().DrawBitmap(
                bitmap.get_raw(),
                &dest_rect.into().into(),
                opacity,
                interpolation as u32,
                &src_rect.into().into(),
            );
        }
    }

    #[inline]
    pub fn draw_text(
        &mut self,
        text: &str,
        format: &TextFormat,
        layout_rect: impl Into<Rectf>,
        foreground_brush: &Brush,
        options: DrawTextOptions,
    ) {
        self.assert_can_draw("draw_text");

        let text = text.to_wide_null();

        unsafe {
            let format = format.get_raw();
            self.rt().DrawText(
                text.as_ptr(),
                text.len() as u32,
                format,
                &layout_rect.into().into(),
                foreground_brush.get_raw(),
                options.0,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    #[inline]
    pub fn draw_text_layout(
        &mut self,
        origin: impl Into<Point2f>,
        layout: &TextLayout,
        brush: &Brush,
        options: DrawTextOptions,
    ) {
        self.assert_can_draw("draw_text_layout");

        unsafe {
            let layout = layout.get_raw();
            self.rt()
                .DrawTextLayout(origin.into().into(), layout, brush.get_raw(), options.0);
        }
    }

    #[inline]
    pub fn set_transform(&mut self, transform: &Matrix3x2f) {
        unsafe { self.rt().SetTransform(transform as *const _ as *const _) }
    }

    #[inline]
    pub fn transform(&self) -> Matrix3x2f {
        unsafe {
            let mut mat: Matrix3x2f = std::mem::uninitialized();
            self.rt().GetTransform(&mut mat as *mut _ as *mut _);
            mat
        }
    }

    #[inline]
    pub fn set_antialias_mode(&mut self, mode: AntialiasMode) {
        unsafe { self.rt().SetAntialiasMode(mode as u32) };
    }

    #[inline]
    pub fn antialias_mode(&mut self) -> UncheckedEnum<AntialiasMode> {
        unsafe { self.rt().GetAntialiasMode().into() }
    }

    #[inline]
    pub fn set_dpi(&mut self, dpi_x: f32, dpi_y: f32) {
        unsafe { self.rt().SetDpi(dpi_x, dpi_y) }
    }

    #[inline]
    pub fn dpi(&self) -> (f32, f32) {
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            self.rt().GetDpi(&mut x, &mut y);
            (x, y)
        }
    }

    #[inline]
    pub fn push_layer<'a, 'b>(&'a mut self, layer: &'b Layer) -> LayerBuilder<'a, 'b>
    where
        Self: Sized + 'a,
    {
        LayerBuilder::create(self, layer)
    }

    #[inline]
    pub fn pop_layer(&mut self) {
        unsafe {
            self.rt().PopLayer();
        }
    }

    #[inline]
    pub fn push_axis_aligned_clip(&mut self, clip: impl Into<Rectf>, aa: AntialiasMode) {
        unsafe {
            self.rt()
                .PushAxisAlignedClip(&clip.into().into(), aa as u32);
        }
    }

    #[inline]
    pub fn pop_axis_aligned_clip(&mut self) {
        unsafe {
            self.rt().PopAxisAlignedClip();
        }
    }
}

// Private fns
impl RenderTarget {
    #[inline]
    unsafe fn make_tag(tag1: D2D1_TAG, tag2: D2D1_TAG) -> Option<RenderTag> {
        if tag1 == 0 {
            None
        } else {
            let tag = RenderTag::from_raw(tag1, tag2);
            Some(tag)
        }
    }

    #[inline]
    pub(crate) unsafe fn rt(&self) -> &ID2D1RenderTarget {
        &*self.ptr
    }

    #[inline]
    pub(crate) fn assert_can_draw(&self, fname: &str) {
        if self.state.0 != 0 {
            let tag = render_tag::fmt_tag(self.get_tag());
            panic!(
                "This RenderTarget is not currently able to be drawn to.\
                 \n    Action: {}\
                 \n    Tag: {}\
                 \n    State: {:?}",
                fname, tag, self.state
            );
        }
    }
}

impl ComWrapper for RenderTarget {
    type Interface = ID2D1RenderTarget;
    unsafe fn get_raw(&self) -> *mut Self::Interface {
        self.ptr.as_raw()
    }
    unsafe fn into_raw(self) -> *mut Self::Interface {
        self.ptr.into_raw()
    }
    unsafe fn from_raw(raw: *mut Self::Interface) -> Self {
        Self::from_ptr(ComPtr::from_raw(raw))
    }
    unsafe fn from_ptr(ptr: ComPtr<Self::Interface>) -> Self {
        RenderTarget {
            ptr,
            state: RTState::NOT_DRAWING,
        }
    }
    unsafe fn into_ptr(self) -> ComPtr<Self::Interface> {
        self.ptr
    }
}

unsafe impl Send for RenderTarget {}
unsafe impl Sync for RenderTarget {}

impl std::ops::Deref for RenderTarget {
    type Target = crate::resource::Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl std::fmt::Debug for RenderTarget {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderTarget")
            .field("ptr", &self.ptr.as_raw())
            .field("state", &self.state)
            .finish()
    }
}

#[enum_flags]
/// Flags that act as blockers to a target being able to be drawn to.
pub(crate) enum RTState {
    NOT_DRAWING,
    NO_TARGET_IMAGE,
}
