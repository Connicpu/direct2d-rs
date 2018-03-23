#![cfg(windows)]

extern crate directwrite;
extern crate dxgi;
extern crate winapi;
extern crate wio;

pub use factory::Factory;
pub use render_target::RenderTarget;

#[macro_use]
mod macros;

pub mod brush;
pub mod device;
pub mod error;
pub mod factory;
pub mod geometry;
pub mod math;
pub mod render_target;
pub mod stroke_style;

mod helpers;
