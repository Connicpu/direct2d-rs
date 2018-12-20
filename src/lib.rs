#![cfg(windows)]

#[macro_use]
extern crate derive_com_wrapper;

#[macro_use]
extern crate auto_enum;

pub use crate::device::Device;
pub use crate::device_context::DeviceContext;
pub use crate::render_target::RenderTarget;

pub mod brush;
pub mod descriptions;
pub mod device;
pub mod device_context;
pub mod enums;
pub mod error;
pub mod factory;
pub mod geometry;
pub mod image;
pub mod layer;
pub mod properties;
pub mod render_target;
pub mod resource;
pub mod stroke_style;
