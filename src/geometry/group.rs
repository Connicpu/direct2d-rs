use enums::FillMode;
use error::D2DResult;
use factory::Factory;
use geometry::{GenericGeometry, Geometry};

use std::{mem, ptr, slice};

use winapi::shared::winerror::{E_FAIL, SUCCEEDED};
use winapi::um::d2d1::{ID2D1Geometry, ID2D1GeometryGroup};
use wio::com::ComPtr;

/// Represents multiple geometries combined into a single item
#[repr(C)]
#[derive(Clone)]
pub struct Group {
    ptr: ComPtr<ID2D1GeometryGroup>,
}

impl Group {
    #[inline]
    pub fn create<G>(factory: &Factory, fill_mode: FillMode, geometry: G) -> D2DResult<Group>
    where
        G: GroupableGeometry,
    {
        let list = geometry.raw_geometry_list();
        let list = list.as_ref();

        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreateGeometryGroup(
                fill_mode as u32,
                list.as_ptr() as *mut _,
                list.len() as u32,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(Group::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn get_fill_mode(&self) -> D2DResult<FillMode> {
        unsafe { FillMode::from_u32(self.ptr.GetFillMode()).ok_or(E_FAIL.into()) }
    }

    #[inline]
    pub fn get_source_geometry_count(&self) -> u32 {
        unsafe { self.ptr.GetSourceGeometryCount() }
    }

    #[inline]
    pub fn get_source_geometries(&self) -> Vec<GenericGeometry> {
        unsafe {
            let count = self.get_source_geometry_count();
            let mut data: Vec<GenericGeometry> = Vec::with_capacity(count as usize);
            self.ptr
                .GetSourceGeometries(data.as_mut_ptr() as *mut _, count);
            data.set_len(count as usize);
            data
        }
    }
}

geometry_type!(Group: ID2D1GeometryGroup);

pub trait GroupableGeometry {
    type List: AsRef<[*mut ID2D1Geometry]>;
    fn raw_geometry_list(self) -> Self::List;
}

impl<'a, G> GroupableGeometry for &'a [G]
where
    G: Geometry,
{
    type List = &'a [*mut ID2D1Geometry];
    #[inline]
    fn raw_geometry_list(self) -> Self::List {
        assert_eq!(mem::size_of::<G>(), mem::size_of::<*mut ID2D1Geometry>());
        unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len()) }
    }
}

impl<'a, G> GroupableGeometry for &'a Vec<G>
where
    G: Geometry,
{
    type List = &'a [*mut ID2D1Geometry];
    #[inline]
    fn raw_geometry_list(self) -> Self::List {
        self[..].raw_geometry_list()
    }
}

macro_rules! groupable_tuples {
    ($($gtup:ident)*) => {
        impl<$($gtup,)*> GroupableGeometry for ($($gtup,)*)
        where
            $($gtup : Geometry,)*
        {
            type List = [*mut ID2D1Geometry; groupable_tuples!(@count_tuple_size $($gtup)*)];
            #[allow(non_snake_case)]
            #[inline]
            fn raw_geometry_list(self) -> Self::List {
                let ($($gtup,)*) = self;
                [$(unsafe{Geometry::get_ptr(&$gtup)},)*]
            }
        }
    };

    (@count_tuple_size) => { 0 };
    (@count_tuple_size $gtup:ident $($rest:ident)*) => { 1 + groupable_tuples!(@count_tuple_size $($rest)*) };
}

groupable_tuples!();
groupable_tuples!(G1);
groupable_tuples!(G1 G2);
groupable_tuples!(G1 G2 G3);
groupable_tuples!(G1 G2 G3 G4);
groupable_tuples!(G1 G2 G3 G4 G5);
groupable_tuples!(G1 G2 G3 G4 G5 G6);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9 G10);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13 G14);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13 G14 G15);
groupable_tuples!(G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13 G14 G15 G16);
