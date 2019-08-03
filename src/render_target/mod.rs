use crate::brush::IBrush;
use crate::enums::{AntialiasMode, BitmapInterpolationMode, DrawTextOptions};
use crate::geometry::IGeometry;
use crate::image::IBitmap;
use crate::layer::{Layer, LayerBuilder};
use crate::resource::IResource;
use crate::stroke_style::StrokeStyle;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::error::Error;
use directwrite::{TextFormat, TextLayout};
use math2d::*;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1RenderTarget, ID2D1Resource, D2D1_TAG};
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

pub unsafe trait IRenderTarget: IResource {
    fn size(&self) -> Sizef {
        unsafe { self.raw_rt().GetSize().into() }
    }

    fn pixel_size(&self) -> Sizeu {
        unsafe { self.raw_rt().GetPixelSize().into() }
    }

    fn begin_draw(&mut self) {
        if !self.draw_state().is_set(RTState::NOT_DRAWING) {
            panic!("You may not call begin_draw() when you are already drawing.");
        }

        unsafe {
            self.raw_rt().BeginDraw();
            self.draw_state_mut().clear(RTState::NOT_DRAWING);
        }
    }

    fn end_draw(&mut self) -> Result<(), (Error, Option<RenderTag>)> {
        self.assert_can_draw("end_draw");

        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            let hr = self.raw_rt().EndDraw(&mut tag1, &mut tag2);
            *self.draw_state_mut() |= RTState::NOT_DRAWING;

            if SUCCEEDED(hr) {
                Ok(())
            } else {
                let tag = make_tag(tag1, tag2);
                Err((From::from(hr), tag))
            }
        }
    }

    fn set_tag(&mut self, tag: Option<RenderTag>) {
        unsafe {
            if let Some(tag) = tag {
                let (tag1, tag2) = tag.to_raw();
                self.raw_rt().SetTags(tag1 as u64, tag2 as u64)
            } else {
                self.raw_rt().SetTags(0, 0);
            }
        };
    }

    fn get_tag(&self) -> Option<RenderTag> {
        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            self.raw_rt().GetTags(&mut tag1, &mut tag2);
            make_tag(tag1, tag2)
        }
    }

    fn flush(&mut self) -> Result<(), (Error, Option<RenderTag>)> {
        self.assert_can_draw("flush");

        let mut tag1 = 0;
        let mut tag2 = 0;
        unsafe {
            let result = self.raw_rt().Flush(&mut tag1, &mut tag2);

            if SUCCEEDED(result) {
                Ok(())
            } else {
                let tag = make_tag(tag1, tag2);
                Err((From::from(result), tag))
            }
        }
    }

    fn clear(&mut self, color: Color) {
        self.assert_can_draw("clear");

        unsafe {
            self.raw_rt().Clear(&color.into());
        }
    }

    fn draw_line(
        &mut self,
        p0: Point2f,
        p1: Point2f,
        brush: &dyn IBrush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_line");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.raw_rt().DrawLine(
                p0.into(),
                p1.into(),
                brush.raw_brush() as *const _ as *mut _,
                stroke_width,
                stroke_style,
            )
        }
    }

    fn draw_rectangle(
        &mut self,
        rect: Rectf,
        brush: &dyn IBrush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_rectangle");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.raw_rt().DrawRectangle(
                &rect.into(),
                brush.raw_brush() as *const _ as *mut _,
                stroke_width,
                stroke_style,
            );
        }
    }

    fn fill_rectangle(&mut self, rect: Rectf, brush: &dyn IBrush) {
        self.assert_can_draw("fill_rectangle");

        unsafe {
            self.raw_rt()
                .FillRectangle(&rect.into(), brush.raw_brush() as *const _ as *mut _);
        }
    }

    fn draw_rounded_rectangle(
        &mut self,
        rect: RoundedRect,
        brush: &dyn IBrush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_rounded_rectangle");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.raw_rt().DrawRoundedRectangle(
                &rect.into(),
                brush.raw_brush() as *const _ as *mut _,
                stroke_width,
                stroke_style,
            );
        }
    }

    fn fill_rounded_rectangle(&mut self, rect: RoundedRect, brush: &dyn IBrush) {
        self.assert_can_draw("fill_rounded_rectangle");

        unsafe {
            self.raw_rt()
                .FillRoundedRectangle(&rect.into(), brush.raw_brush() as *const _ as *mut _);
        }
    }

    fn draw_ellipse(
        &mut self,
        ellipse: Ellipse,
        brush: &dyn IBrush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_ellipse");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.raw_rt().DrawEllipse(
                &ellipse.into(),
                brush.raw_brush() as *const _ as *mut _,
                stroke_width,
                stroke_style,
            );
        }
    }

    fn fill_ellipse(&mut self, ellipse: Ellipse, brush: &dyn IBrush) {
        self.assert_can_draw("fill_ellipse");

        unsafe {
            self.raw_rt()
                .FillEllipse(&ellipse.into(), brush.raw_brush() as *const _ as *mut _);
        }
    }

    fn draw_geometry(
        &mut self,
        geometry: &dyn IGeometry,
        brush: &dyn IBrush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        self.assert_can_draw("draw_geometry");

        unsafe {
            let stroke_style = match stroke_style {
                Some(s) => s.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            self.raw_rt().DrawGeometry(
                geometry.raw_geom() as *const _ as *mut _,
                brush.raw_brush() as *const _ as *mut _,
                stroke_width,
                stroke_style,
            );
        }
    }

    fn fill_geometry(&mut self, geometry: &dyn IGeometry, brush: &dyn IBrush) {
        self.assert_can_draw("fill_geometry");

        unsafe {
            self.raw_rt().FillGeometry(
                geometry.raw_geom() as *const _ as *mut _,
                brush.raw_brush() as *const _ as *mut _,
                std::ptr::null_mut(),
            );
        }
    }

    fn fill_geometry_with_opacity(
        &mut self,
        geometry: &dyn IGeometry,
        brush: &dyn IBrush,
        opacity_brush: &dyn IBrush,
    ) {
        self.assert_can_draw("fill_geometry_with_opacity");

        unsafe {
            self.raw_rt().FillGeometry(
                geometry.raw_geom() as *const _ as *mut _,
                brush.raw_brush() as *const _ as *mut _,
                opacity_brush.raw_brush() as *const _ as *mut _,
            );
        }
    }

    fn draw_bitmap(
        &mut self,
        bitmap: &dyn IBitmap,
        dest_rect: Rectf,
        opacity: f32,
        interpolation: BitmapInterpolationMode,
        src_rect: Rectf,
    ) {
        self.assert_can_draw("draw_bitmap");

        unsafe {
            self.raw_rt().DrawBitmap(
                bitmap.raw_bitmap() as *const _ as *mut _,
                &dest_rect.into(),
                opacity,
                interpolation as u32,
                &src_rect.into(),
            );
        }
    }

    fn draw_text(
        &mut self,
        text: &str,
        format: &TextFormat,
        layout_rect: Rectf,
        foreground_brush: &dyn IBrush,
        options: DrawTextOptions,
    ) {
        self.assert_can_draw("draw_text");

        let text = text.to_wide_null();

        unsafe {
            let format = format.get_raw();
            self.raw_rt().DrawText(
                text.as_ptr(),
                text.len() as u32,
                format,
                &layout_rect.into(),
                foreground_brush.raw_brush() as *const _ as *mut _,
                options.0,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    fn draw_text_layout(
        &mut self,
        origin: Point2f,
        layout: &TextLayout,
        brush: &dyn IBrush,
        options: DrawTextOptions,
    ) {
        self.assert_can_draw("draw_text_layout");

        unsafe {
            let layout = layout.get_raw();
            self.raw_rt().DrawTextLayout(
                origin.into(),
                layout,
                brush.raw_brush() as *const _ as *mut _,
                options.0,
            );
        }
    }

    fn set_transform(&mut self, transform: &Matrix3x2f) {
        unsafe {
            self.raw_rt()
                .SetTransform(transform as *const _ as *const _)
        }
    }

    fn transform(&self) -> Matrix3x2f {
        unsafe {
            let mut mat: Matrix3x2f = std::mem::uninitialized();
            self.raw_rt().GetTransform(&mut mat as *mut _ as *mut _);
            mat
        }
    }

    fn set_antialias_mode(&mut self, mode: AntialiasMode) {
        unsafe { self.raw_rt().SetAntialiasMode(mode as u32) };
    }

    fn antialias_mode(&mut self) -> UncheckedEnum<AntialiasMode> {
        unsafe { self.raw_rt().GetAntialiasMode().into() }
    }

    fn set_dpi(&mut self, dpi_x: f32, dpi_y: f32) {
        unsafe { self.raw_rt().SetDpi(dpi_x, dpi_y) }
    }

    fn dpi(&self) -> (f32, f32) {
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            self.raw_rt().GetDpi(&mut x, &mut y);
            (x, y)
        }
    }

    fn push_layer<'a, 'b>(&'a mut self, layer: &'b Layer) -> LayerBuilder<'a, 'b>
    where
        Self: Sized,
    {
        LayerBuilder::create(self, layer)
    }

    fn pop_layer(&mut self) {
        unsafe {
            self.raw_rt().PopLayer();
        }
    }

    fn push_axis_aligned_clip(&mut self, clip: Rectf, aa: AntialiasMode) {
        unsafe {
            self.raw_rt().PushAxisAlignedClip(&clip.into(), aa as u32);
        }
    }

    fn pop_axis_aligned_clip(&mut self) {
        unsafe {
            self.raw_rt().PopAxisAlignedClip();
        }
    }

    unsafe fn raw_rt(&self) -> &ID2D1RenderTarget;
    fn draw_state(&self) -> RTState;
    fn draw_state_mut(&mut self) -> &mut RTState;

    #[doc(hidden)]
    fn assert_can_draw(&self, fname: &str) {
        let state = self.draw_state();
        if state.0 != 0 {
            let tag = render_tag::fmt_tag(self.get_tag());
            panic!(
                "This RenderTarget is not currently able to be drawn to.\
                 \n    Action: {}\
                 \n    Tag: {}\
                 \n    State: {:?}",
                fname, tag, state
            );
        }
    }
}

impl dyn IRenderTarget + '_ {
    pub fn push_layer_dyn<'a, 'b>(&'a mut self, layer: &'b Layer) -> LayerBuilder<'a, 'b> {
        LayerBuilder::create(self, layer)
    }
}

unsafe impl IResource for RenderTarget {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IRenderTarget for RenderTarget {
    unsafe fn raw_rt(&self) -> &ID2D1RenderTarget {
        &self.ptr
    }

    fn draw_state(&self) -> RTState {
        self.state
    }

    fn draw_state_mut(&mut self) -> &mut RTState {
        &mut self.state
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
pub(crate) unsafe fn make_tag(tag1: D2D1_TAG, tag2: D2D1_TAG) -> Option<RenderTag> {
    if tag1 == 0 {
        None
    } else {
        let tag = RenderTag::from_raw(tag1, tag2);
        Some(tag)
    }
}

unsafe impl Send for RenderTarget {}
unsafe impl Sync for RenderTarget {}

impl std::fmt::Debug for RenderTarget {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderTarget")
            .field("ptr", &self.ptr.as_raw())
            .field("state", &self.state)
            .finish()
    }
}

#[auto_enum::enum_flags]
/// Flags that act as blockers to a target being able to be drawn to.
pub enum RTState {
    NOT_DRAWING,
    NO_TARGET_IMAGE,
}
