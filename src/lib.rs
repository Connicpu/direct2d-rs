#![feature(slice_patterns, const_fn)]

extern crate winapi;
extern crate kernel32;
extern crate uuid;

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

mod comptr;
mod load_dll;
mod helpers;
