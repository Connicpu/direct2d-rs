#![cfg(windows)]
#![feature(slice_patterns, const_fn)]

extern crate winapi;
extern crate uuid;
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
pub mod comptr;

mod load_dll;
mod helpers;
