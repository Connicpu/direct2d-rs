#[derive(Copy, Clone, Debug)]
pub struct RenderTag {
    pub loc: &'static str,
}

impl std::fmt::Display for RenderTag {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "'{}'", self.loc)
    }
}

impl RenderTag {
    pub fn to_raw(&self) -> (u64, u64) {
        let ptr = self.loc.as_ptr() as usize;
        let len = self.loc.len();
        (ptr as u64, len as u64)
    }

    pub unsafe fn from_raw(tag1: u64, tag2: u64) -> RenderTag {
        let ptr = tag1 as usize as *const u8;
        let len = tag2 as usize;
        RenderTag {
            loc: std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)),
        }
    }
}

pub(crate) fn fmt_tag(tag: Option<RenderTag>) -> impl std::fmt::Display {
    struct DRT(Option<RenderTag>);
    impl std::fmt::Display for DRT {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self.0 {
                Some(tag) => write!(fmt, "'{}'", tag.loc),
                None => fmt.write_str("None"),
            }
        }
    }
    DRT(tag)
}

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
///         .with_color(0xFF_7F_7F)
///         .build().unwrap();
///
///     context.begin_draw();
///     context.set_target(target);
///     context.clear(0xFF_FF_FF);
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
/// #     use direct3d11::enums::{BindFlags, CreateDeviceFlags};
/// #     use dxgi::enums::Format;
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
    ($rt:expr) => {
        $crate::render_target::RenderTarget::set_tag(
            $rt,
            Some($crate::make_render_tag!()),
        );
    };
}
