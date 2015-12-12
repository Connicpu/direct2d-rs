use std::sync::Arc;
use winapi::*;
use comptr::ComPtr;
use load_dll;
use error::D2D1Error;
use helpers::{GetRaw, FromRaw};
use geometry;
use math;

#[derive(Clone, Debug, PartialEq)]
pub struct Factory {
    ptr: ComPtr<ID2D1Factory>,
}

impl GetRaw for Factory {
    type Raw = ID2D1Factory;
    unsafe fn get_raw(&self) -> *mut ID2D1Factory {
        self.ptr.raw_value()
    }
}

impl Factory {
    pub unsafe fn from_ptr(ptr: ComPtr<ID2D1Factory>) -> Factory {
        Factory {
            ptr: ptr
        }
    }
    
    pub fn create() -> Result<Factory, D2D1Error> {
        let d2d1 = match load_dll::D2D1::load() {
            Ok(d2d1) => Arc::new(d2d1),
            Err(_) => return Err(D2D1Error::MissingLibrary),
        };
        
        let mut ptr: ComPtr<ID2D1Factory> = ComPtr::new();
        unsafe {
            let hr = d2d1.create_factory(
                D2D1_FACTORY_TYPE_MULTI_THREADED,
                &ptr.iid(),
                &D2D1_FACTORY_OPTIONS {
                    debugLevel: D2D1_DEBUG_LEVEL_WARNING
                },
                ptr.raw_void()
            );
            
            if !SUCCEEDED(hr) {
                return Err(From::from(hr));
            }
        }
        
        Ok(Factory {
            ptr: ptr,
        })
    }
    
    pub fn create_rectangle_geometry(
        &self, rectangle: &math::RectF
    ) -> Result<geometry::Rectangle, D2D1Error> {
        unsafe {
            let mut ptr: ComPtr<ID2D1RectangleGeometry> = ComPtr::new();
            let result = (*self.ptr.raw_value()).CreateRectangleGeometry(&rectangle.0, ptr.raw_addr());
            
            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr.raw_value()))
            } else {
                Err(From::from(result))
            }
        }
    }
    
    pub fn create_rounded_rectangle_geometry(
        &self, rounded_rectangle: &math::RoundedRect
    ) -> Result<geometry::RoundedRectangle, D2D1Error> {
        unsafe {
            let mut ptr: ComPtr<ID2D1RoundedRectangleGeometry> = ComPtr::new();
            let result = (*self.ptr.raw_value()).CreateRoundedRectangleGeometry(
                &rounded_rectangle.0, ptr.raw_addr()
            );
            
            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr.raw_value()))
            } else {
                Err(From::from(result))
            }
        }
    }
    
    pub fn create_ellipse_geometry(
        &self, ellipse: &math::Ellipse
    ) -> Result<geometry::Ellipse, D2D1Error> {
        unsafe {
            let mut ptr: ComPtr<ID2D1EllipseGeometry> = ComPtr::new();
            let result = (*self.ptr.raw_value()).CreateEllipseGeometry(&ellipse.0, ptr.raw_addr());
            
            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr.raw_value()))
            } else {
                Err(From::from(result))
            }
        }
    }
    
    pub fn create_geometry_group<G: geometry::Geometry>(
        &self, fill_mode: geometry::FillMode, geometries: &[G]
    ) -> Result<geometry::Group, D2D1Error> {
        unsafe {
            let mut ptrs: Vec<_> = geometries.iter().map(|g| g.get_ptr()).collect();
            let mut ptr: ComPtr<ID2D1GeometryGroup> = ComPtr::new();
            
            let result = (*self.ptr.raw_value()).CreateGeometryGroup(
                D2D1_FILL_MODE(fill_mode as u32),
                ptrs.as_mut_ptr(),
                ptrs.len() as u32,
                ptr.raw_addr(),
            );
            
            if SUCCEEDED(result) {
                Ok(FromRaw::from_raw(ptr.raw_value()))
            } else {
                Err(From::from(result))
            }
        }
    }
}
