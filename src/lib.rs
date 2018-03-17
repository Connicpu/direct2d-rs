#![cfg(windows)]

extern crate winapi;
extern crate wio;
extern crate directwrite;

pub use factory::Factory;
pub use render_target::RenderTarget;

#[macro_use]
mod macros;

pub mod factory;
pub mod render_target;
pub mod error;
pub mod math;
pub mod geometry;
pub mod stroke_style;
pub mod brush;

mod load_dll;
mod helpers;
