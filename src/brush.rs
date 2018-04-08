use factory::Factory;
use helpers::{FromRaw, GetRaw};
use math::*;

use std::{mem, ptr};

use winapi::um::d2d1::{ID2D1Brush, ID2D1Factory, ID2D1GradientStopCollection,
                       ID2D1LinearGradientBrush, ID2D1SolidColorBrush};
use wio::com::ComPtr;

pub trait Brush {
    unsafe fn get_ptr(&self) -> *mut ID2D1Brush;

    fn get_factory(&self) -> Factory {
        unsafe {
            let mut ptr: *mut ID2D1Factory = ptr::null_mut();
            (*self.get_ptr()).GetFactory(&mut ptr);

            Factory::from_ptr(ComPtr::from_raw(ptr).cast().unwrap())
        }
    }

    fn to_generic(&self) -> GenericBrush {
        let ptr = unsafe { ComPtr::from_raw(self.get_ptr()) };
        mem::forget(ptr.clone());
        GenericBrush { ptr }
    }

    fn set_opacity(&mut self, opacity: f32) {
        unsafe {
            (*self.get_ptr()).SetOpacity(opacity);
        }
    }

    fn set_transform(&mut self, transform: &Matrix3x2F) {
        unsafe {
            (*self.get_ptr()).SetTransform(&transform.0);
        }
    }

    fn get_opacity(&self) -> f32 {
        unsafe { (*self.get_ptr()).GetOpacity() }
    }

    fn get_transform(&self) -> Matrix3x2F {
        unsafe {
            let mut mat: Matrix3x2F = mem::uninitialized();
            (*self.get_ptr()).GetTransform(&mut mat.0);
            mat
        }
    }
}

brush_types! {
    pub struct GenericBrush(ID2D1Brush);
    pub struct SolidColor(ID2D1SolidColorBrush);
    pub struct LinearGradientBrush(ID2D1LinearGradientBrush);
}

impl SolidColor {
    pub fn set_color(&mut self, color: &ColorF) {
        unsafe { self.ptr.SetColor(&color.0) };
    }

    pub fn get_color(&self) -> ColorF {
        unsafe { ColorF(self.ptr.GetColor()) }
    }
}

impl LinearGradientBrush {
    pub fn get_start_point(&self) -> Point2F {
        unsafe { Point2F(self.ptr.GetStartPoint()) }
    }

    pub fn get_end_point(&self) -> Point2F {
        unsafe { Point2F(self.ptr.GetEndPoint()) }
    }

    pub fn get_gradient_stop_collection(&self) -> GradientStopCollection {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetGradientStopCollection(&mut ptr);
            GradientStopCollection {
                ptr: ComPtr::from_raw(ptr),
            }
        }
    }

    pub fn set_start_point(&mut self, point: Point2F) {
        unsafe { self.ptr.SetStartPoint(point.0) }
    }

    pub fn set_end_point(&mut self, point: Point2F) {
        unsafe { self.ptr.SetEndPoint(point.0) }
    }
}

pub struct GradientStopCollection {
    ptr: ComPtr<ID2D1GradientStopCollection>,
}

impl FromRaw for GradientStopCollection {
    type Raw = ID2D1GradientStopCollection;
    unsafe fn from_raw(raw: *mut ID2D1GradientStopCollection) -> Self {
        GradientStopCollection {
            ptr: ComPtr::from_raw(raw),
        }
    }
}

impl GetRaw for GradientStopCollection {
    type Raw = ID2D1GradientStopCollection;
    unsafe fn get_raw(&self) -> *mut ID2D1GradientStopCollection {
        self.ptr.as_raw()
    }
}

impl GradientStopCollection {
    pub fn len(&self) -> u32 {
        unsafe { self.ptr.GetGradientStopCount() }
    }

    pub fn get_stops(&self) -> Vec<GradientStop> {
        unsafe {
            let len = self.len();
            let mut stops: Vec<GradientStop> = Vec::with_capacity(len as usize);
            self.ptr.GetGradientStops(stops.as_mut_ptr() as *mut _, len);
            stops
        }
    }
}

#[repr(C)]
pub struct GradientStop {
    pub position: f32,
    pub color: ColorF,
}

pub enum ExtendMode {
    Clamp = 0,
    Wrap = 1,
    Mirror = 2,
}
