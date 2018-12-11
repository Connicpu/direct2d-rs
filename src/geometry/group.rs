use crate::enums::{FillMode};
use crate::error::D2DResult;
use crate::factory::Factory;
use crate::geometry::{GenericGeometry, Geometry};

use std::{mem, ptr, slice};

use checked_enum::UncheckedEnum;
use winapi::shared::winerror::SUCCEEDED;
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
    pub fn get_fill_mode(&self) -> UncheckedEnum<FillMode> {
        unsafe { self.ptr.GetFillMode().into() }
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

#[doc(hidden)]
pub enum MaybeOwnedGeomList<'a> {
    Ref(&'a [*mut ID2D1Geometry]),
    Own(Vec<*mut ID2D1Geometry>),
}

impl<'a> AsRef<[*mut ID2D1Geometry]> for MaybeOwnedGeomList<'a> {
    fn as_ref(&self) -> &[*mut ID2D1Geometry] {
        match *self {
            MaybeOwnedGeomList::Ref(slice) => slice,
            MaybeOwnedGeomList::Own(ref vec) => vec,
        }
    }
}

impl<'a, G> GroupableGeometry for &'a [G]
where
    G: Geometry,
{
    type List = MaybeOwnedGeomList<'a>;
    #[inline]
    fn raw_geometry_list(self) -> Self::List {
        unsafe {
            if mem::size_of::<G>() == mem::size_of::<*mut ID2D1Geometry>() && self.len() > 0
                && mem::transmute_copy::<G, *mut ID2D1Geometry>(&self[0]) == self[0].get_ptr()
            {
                MaybeOwnedGeomList::Ref(slice::from_raw_parts(
                    self.as_ptr() as *const _,
                    self.len(),
                ))
            } else {
                MaybeOwnedGeomList::Own(self.iter().map(|g| g.get_ptr()).collect())
            }
        }
    }
}

impl<'a, G> GroupableGeometry for &'a Vec<G>
where
    G: Geometry,
{
    type List = <&'a [G] as GroupableGeometry>::List;
    #[inline]
    fn raw_geometry_list(self) -> Self::List {
        self[..].raw_geometry_list()
    }
}

macro_rules! groupable_variadic {
    (@tup $($gtup:ident)*) => {
        impl<$($gtup,)*> GroupableGeometry for ($($gtup,)*)
        where
            $($gtup : Geometry,)*
        {
            type List = [*mut ID2D1Geometry; groupable_variadic!(@count_tuple_size $($gtup)*)];
            #[allow(non_snake_case)]
            #[inline]
            fn raw_geometry_list(self) -> Self::List {
                let ($($gtup,)*) = self;
                [$(unsafe { Geometry::get_ptr(&$gtup) },)*]
            }
        }
    };

    (@arrays $($len:expr)*) => {$(
        impl<G> GroupableGeometry for [G; $len] where G: Geometry {
            type List = [*mut ID2D1Geometry; $len];
            #[inline]
            fn raw_geometry_list(self) -> Self::List {
                GroupableGeometry::raw_geometry_list(&self)
            }
        }
        impl<'a, G> GroupableGeometry for &'a [G; $len] where G: Geometry + 'a {
            type List = [*mut ID2D1Geometry; $len];
            #[inline]
            fn raw_geometry_list(self) -> Self::List {
                let mut data = [ptr::null_mut(); $len];
                for (src, dst) in self.iter().zip(data.iter_mut()) {
                    *dst = unsafe { src.get_ptr() };
                }
                data
            }
        }
    )*};

    (@count_tuple_size) => { 0 };
    (@count_tuple_size $gtup:ident $($rest:ident)*) => { 1 + groupable_variadic!(@count_tuple_size $($rest)*) };
}

groupable_variadic!(@tup );
groupable_variadic!(@tup G1);
groupable_variadic!(@tup G1 G2);
groupable_variadic!(@tup G1 G2 G3);
groupable_variadic!(@tup G1 G2 G3 G4);
groupable_variadic!(@tup G1 G2 G3 G4 G5);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9 G10);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13 G14);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13 G14 G15);
groupable_variadic!(@tup G1 G2 G3 G4 G5 G6 G7 G8 G9 G10 G11 G12 G13 G14 G15 G16);
groupable_variadic!(@arrays 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
