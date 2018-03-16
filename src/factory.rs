use std::ptr;
use std::sync::Arc;
use wio::com::ComPtr;
use load_dll;
use error::D2D1Error;
use helpers::{FromRaw, GetRaw};
use render_target::{RenderTarget, RenderTargetBacking};
use geometry;
use stroke_style;
use math;

use winapi::Interface;
use winapi::ctypes::c_void;
use winapi::shared::winerror::*;
use winapi::um::d2d1::*;
use winapi::um::d2d1_1::*;

#[derive(Clone, PartialEq)]
pub struct Factory {
    ptr: ComPtr<ID2D1Factory1>,
}

impl GetRaw for Factory {
    type Raw = ID2D1Factory1;
    unsafe fn get_raw(&self) -> *mut ID2D1Factory1 {
        self.ptr.as_raw()
    }
}

impl Factory {
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1Factory1>) -> Factory {
        Factory { ptr: ptr }
    }

    pub fn new() -> Result<Factory, D2D1Error> {
        let d2d1 = match load_dll::D2D1::load() {
            Ok(d2d1) => Arc::new(d2d1),
            Err(_) => return Err(D2D1Error::MissingLibrary),
        };

        let mut ptr: *mut ID2D1Factory1 = ptr::null_mut();
        unsafe {
            let hr = d2d1.create_factory(
                D2D1_FACTORY_TYPE_MULTI_THREADED,
                &ID2D1Factory::uuidof(),
                &D2D1_FACTORY_OPTIONS {
                    debugLevel: D2D1_DEBUG_LEVEL_WARNING,
                },
                &mut ptr as *mut _ as *mut *mut c_void,
            );

            if !SUCCEEDED(hr) {
                return Err(From::from(hr));
            }

            Ok(Factory { ptr: ComPtr::from_raw(ptr) })
        }
    }

    pub fn create_rectangle_geometry(
        &self,
        rectangle: &math::RectF,
    ) -> Result<geometry::Rectangle, D2D1Error> {
        unsafe {
            let mut ptr: *mut ID2D1RectangleGeometry = ptr::null_mut();
            let result =
                self.ptr.CreateRectangleGeometry(&rectangle.0, &mut ptr);

            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr))
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn create_rounded_rectangle_geometry(
        &self,
        rounded_rectangle: &math::RoundedRect,
    ) -> Result<geometry::RoundedRectangle, D2D1Error> {
        unsafe {
            let mut ptr: *mut ID2D1RoundedRectangleGeometry = ptr::null_mut();
            let result = self.ptr
                .CreateRoundedRectangleGeometry(&rounded_rectangle.0, &mut ptr);

            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr))
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn create_ellipse_geometry(
        &self,
        ellipse: &math::Ellipse,
    ) -> Result<geometry::Ellipse, D2D1Error> {
        unsafe {
            let mut ptr: *mut ID2D1EllipseGeometry = ptr::null_mut();
            let result = self.ptr.CreateEllipseGeometry(&ellipse.0, &mut ptr);

            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr))
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn create_geometry_group<G: geometry::Geometry>(
        &self,
        fill_mode: geometry::FillMode,
        geometries: &[G],
    ) -> Result<geometry::Group, D2D1Error> {
        unsafe {
            let mut ptrs: Vec<_> = geometries.iter().map(|g| g.get_ptr()).collect();
            let mut ptr: *mut ID2D1GeometryGroup = ptr::null_mut();

            let result = self.ptr.CreateGeometryGroup(
                fill_mode as u32,
                ptrs.as_mut_ptr(),
                ptrs.len() as u32,
                &mut ptr,
            );

            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr))
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn create_path_geometry(&self) -> Result<geometry::Path, D2D1Error> {
        unsafe {
            let mut ptr: *mut ID2D1PathGeometry1 = ptr::null_mut();
            let result = self.ptr.CreatePathGeometry(&mut ptr);

            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr))
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn create_stroke_style(
        &self,
        props: &stroke_style::StrokeStyleProperties,
    ) -> Result<stroke_style::StrokeStyle, D2D1Error> {
        unsafe {
            let mut ptr: *mut ID2D1StrokeStyle1 = ptr::null_mut();
            let pdata = props.get_d2d1_data();
            let (dashes, dashes_count) = match props.dashes {
                Some(dashes) => (dashes.as_ptr(), dashes.len() as u32),
                None => (ptr::null(), 0),
            };

            let result = self.ptr
                .CreateStrokeStyle(&pdata, dashes, dashes_count, &mut ptr);

            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr))
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn create_render_target<T: RenderTargetBacking>(
        &self,
        backing: T,
    ) -> Result<RenderTarget, D2D1Error> {
        unsafe {
            let factory = &mut *self.ptr.as_raw();
            let rt_ptr = try!(backing.create_target(factory));
            assert!(!rt_ptr.is_null());

            (*rt_ptr).SetTags(0, 0);

            Ok(FromRaw::from_raw(rt_ptr))
        }
    }
}
