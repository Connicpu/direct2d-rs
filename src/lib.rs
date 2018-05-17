#![cfg_attr(
    not(windows),
    doc = "You are viewing this documentation for a platform that isn't windows. You might wanna \
           [switch](https://docs.rs/direct2d/*/x86_64-pc-windows-msvc/direct2d/)\\^\\^"
)]
#![cfg_attr(feature = "docs", feature(external_doc))]
#![cfg_attr(all(windows, feature = "docs"), doc(include = "../CRATE_README.md"))]

#[cfg(windows)]
extern crate directwrite;
#[cfg(windows)]
extern crate dxgi;
#[cfg(windows)]
extern crate either;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate wio;

#[cfg(windows)]
pub use brush::Brush;
#[cfg(windows)]
pub use device::Device;
#[cfg(windows)]
pub use device_context::DeviceContext;
#[cfg(windows)]
pub use error::{D2DResult, Error};
#[cfg(windows)]
pub use factory::Factory;
#[cfg(windows)]
pub use geometry::Geometry;
#[cfg(windows)]
pub use render_target::RenderTarget;
#[cfg(windows)]
pub use stroke_style::StrokeStyle;

#[cfg(windows)]
#[macro_use]
mod macros;

#[cfg(windows)]
pub mod brush;
#[cfg(windows)]
pub mod device;
#[cfg(windows)]
pub mod device_context;
#[cfg(windows)]
pub mod enums;
#[cfg(windows)]
pub mod error;
#[cfg(windows)]
pub mod factory;
#[cfg(windows)]
pub mod geometry;
#[cfg(windows)]
pub mod image;
#[cfg(windows)]
pub mod math;
#[cfg(windows)]
pub mod render_target;
#[cfg(windows)]
pub mod stroke_style;
