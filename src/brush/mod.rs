use factory::Factory;
use math::*;

use std::{mem, ptr};

use winapi::um::d2d1::ID2D1Brush;
use winapi::um::d2d1_1::ID2D1Factory1;
use wio::com::ComPtr;

pub use brush::bitmap::BitmapBrush;
pub use brush::gradient::linear::LinearGradientBrush;
pub use brush::gradient::radial::RadialGradientBrush;
pub use brush::gradient::{GradientStop, GradientStopCollection};
pub use brush::solid_color::SolidColorBrush;

pub mod bitmap;
pub mod gradient;
pub mod solid_color;

/// Defines an object that paints an area. Interfaces that implement Brush describe how the area
/// is painted.
pub trait Brush {
    /// Gets the [Factory][1] instance with which this Brush is associated
    ///
    /// [1]: ../factory/struct.Factory.html
    #[inline]
    fn get_factory(&self) -> Factory {
        unsafe {
            let mut ptr = ptr::null_mut();
            (*self.get_ptr()).GetFactory(&mut ptr);

            let ptr: ComPtr<ID2D1Factory1> = ComPtr::from_raw(ptr).cast().unwrap();
            Factory::from_raw(ptr.into_raw())
        }
    }

    /// Converts the brush to a concrete reference to a brush of unknown type
    #[inline]
    fn to_generic(&self) -> GenericBrush {
        let ptr = unsafe { ComPtr::from_raw(self.get_ptr()) };
        mem::forget(ptr.clone());
        GenericBrush { ptr }
    }

    /// Sets the opacity of the brush, with 1.0 being fully opaque
    #[inline]
    fn set_opacity(&mut self, opacity: f32) {
        unsafe {
            (*self.get_ptr()).SetOpacity(opacity);
        }
    }

    /// Applies a transformation to the brush. This does nothing for a solid color brush, but
    /// can scale, skew, offset, or rotate other brush types such as gradients or images.
    #[inline]
    fn set_transform(&mut self, transform: &Matrix3x2F) {
        unsafe {
            (*self.get_ptr()).SetTransform(&transform.0);
        }
    }

    /// Gets the opacity value you set
    #[inline]
    fn get_opacity(&self) -> f32 {
        unsafe { (*self.get_ptr()).GetOpacity() }
    }

    /// Gets the transform you set
    #[inline]
    fn get_transform(&self) -> Matrix3x2F {
        unsafe {
            let mut mat: Matrix3x2F = mem::uninitialized();
            (*self.get_ptr()).GetTransform(&mut mat.0);
            mat
        }
    }

    /// Gets the raw pointer to the brush
    unsafe fn get_ptr(&self) -> *mut ID2D1Brush;
}

/// A reference to a brush that could be any derived type
#[derive(Clone)]
pub struct GenericBrush {
    ptr: ComPtr<ID2D1Brush>,
}

brush_type!(GenericBrush: ID2D1Brush);

impl GenericBrush {
    /// Try upcasting to a Solid Color
    pub fn as_solid_color(&self) -> Option<SolidColorBrush> {
        unsafe { Some(SolidColorBrush::from_ptr(self.ptr.cast().ok()?)) }
    }

    /// Try upcasting to a Bitmap
    pub fn as_bitmap(&self) -> Option<BitmapBrush> {
        unsafe { Some(BitmapBrush::from_ptr(self.ptr.cast().ok()?)) }
    }

    /// Try upcasting to a Linear Gradient
    pub fn as_linear_gadient(&self) -> Option<LinearGradientBrush> {
        unsafe { Some(LinearGradientBrush::from_ptr(self.ptr.cast().ok()?)) }
    }

    /// Try upcasting to a Radial Gradient
    pub fn as_radial_gadient(&self) -> Option<RadialGradientBrush> {
        unsafe { Some(RadialGradientBrush::from_ptr(self.ptr.cast().ok()?)) }
    }
}
