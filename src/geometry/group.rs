use crate::enums::FillMode;
use crate::factory::IFactory;
use crate::geometry::{Geometry, GeometryType, IGeometry};
use crate::resource::IResource;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1Geometry, ID2D1GeometryGroup, ID2D1Resource};
use wio::com::ComPtr;

pub use self::grouppable::*;

pub mod grouppable;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents multiple geometries combined into a single item
pub struct GroupGeometry {
    ptr: ComPtr<ID2D1GeometryGroup>,
}

impl GroupGeometry {
    pub fn create<'a>(
        factory: &dyn IFactory,
        fill_mode: FillMode,
        geometry: &'a impl GroupableGeometry<'a>,
    ) -> Result<GroupGeometry, Error> {
        let list = geometry.raw_geometry_list();
        let list = list.as_ref();

        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = factory.raw_f().CreateGeometryGroup(
                fill_mode as u32,
                list.as_ptr() as *mut *mut _,
                list.len() as u32,
                &mut ptr,
            );

            eprintln!("made?");

            if SUCCEEDED(hr) {
                Ok(GroupGeometry::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn fill_mode(&self) -> UncheckedEnum<FillMode> {
        unsafe { self.ptr.GetFillMode().into() }
    }

    pub fn source_geometry_count(&self) -> u32 {
        unsafe { self.ptr.GetSourceGeometryCount() }
    }

    pub fn source_geometries(&self) -> Vec<Geometry> {
        unsafe {
            let count = self.source_geometry_count();
            let mut data: Vec<Geometry> = Vec::with_capacity(count as usize);
            self.ptr
                .GetSourceGeometries(data.as_mut_ptr() as *mut _, count);
            data.set_len(count as usize);
            data
        }
    }
}

unsafe impl IResource for GroupGeometry {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IGeometry for GroupGeometry {
    unsafe fn raw_geom(&self) -> &ID2D1Geometry {
        &self.ptr
    }
}

unsafe impl GeometryType for GroupGeometry {}
