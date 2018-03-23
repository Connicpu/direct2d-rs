use std::{mem, ptr};
use math::*;
use wio::com::ComPtr;
use factory::Factory;

use winapi::um::d2d1::*;

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
}

impl SolidColor {
    pub fn set_color(&mut self, color: &ColorF) {
        unsafe { self.ptr.SetColor(&color.0) };
    }

    pub fn get_color(&self) -> ColorF {
        unsafe { ColorF(self.ptr.GetColor()) }
    }
}
