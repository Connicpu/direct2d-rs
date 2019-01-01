use super::GeometryType;
use crate::enums::FillMode;
use crate::error::D2DResult;
use crate::factory::Factory;
use crate::geometry::Geometry;

use std::ptr;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::helpers::deref_com_wrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1GeometryGroup;
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
    #[inline]
    pub fn create<'a>(
        factory: &Factory,
        fill_mode: FillMode,
        geometry: &'a impl GroupableGeometry<'a>,
    ) -> D2DResult<GroupGeometry> {
        let list = geometry.raw_geometry_list();
        let list = list.as_ref();

        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateGeometryGroup(
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

    #[inline]
    pub fn fill_mode(&self) -> UncheckedEnum<FillMode> {
        unsafe { self.ptr.GetFillMode().into() }
    }

    #[inline]
    pub fn source_geometry_count(&self) -> u32 {
        unsafe { self.ptr.GetSourceGeometryCount() }
    }

    #[inline]
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

impl std::ops::Deref for GroupGeometry {
    type Target = super::Geometry;
    fn deref(&self) -> &super::Geometry {
        unsafe { deref_com_wrapper(self) }
    }
}

unsafe impl super::GeometryType for GroupGeometry {}
