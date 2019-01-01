use super::GeometryType;

use std::{mem, ptr, slice};

use winapi::um::d2d1::ID2D1Geometry;

pub unsafe trait GroupableGeometry<'a> {
    type List: AsRef<[*mut ID2D1Geometry]> + 'a;
    fn raw_geometry_list(&'a self) -> Self::List;
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

unsafe impl<'a, G> GroupableGeometry<'a> for [G]
where
    G: GeometryType,
{
    type List = MaybeOwnedGeomList<'a>;
    #[inline]
    fn raw_geometry_list(&'a self) -> Self::List {
        unsafe {
            if mem::size_of::<G>() == mem::size_of::<*mut ID2D1Geometry>()
                && self.len() > 0
                && mem::transmute_copy::<G, *mut ID2D1Geometry>(&self[0]) == self[0].get_raw() as _
            {
                MaybeOwnedGeomList::Ref(slice::from_raw_parts(
                    self.as_ptr() as *const _,
                    self.len(),
                ))
            } else {
                MaybeOwnedGeomList::Own(self.iter().map(|g| g.get_raw() as _).collect())
            }
        }
    }
}

unsafe impl<'a, G> GroupableGeometry<'a> for Vec<G>
where
    G: GeometryType,
{
    type List = <[G] as GroupableGeometry<'a>>::List;
    #[inline]
    fn raw_geometry_list(&'a self) -> Self::List {
        self[..].raw_geometry_list()
    }
}

macro_rules! groupable_variadic {
    (@tup $($gtup:ident)*) => {
        unsafe impl<'a, $($gtup,)*> GroupableGeometry<'a> for ($($gtup,)*)
        where
            $($gtup : GeometryType,)*
        {
            type List = [*mut ID2D1Geometry; groupable_variadic!(@count_tuple_size $($gtup)*)];
            #[allow(non_snake_case)]
            #[inline]
            fn raw_geometry_list(&'a self) -> Self::List {
                let ($(ref $gtup,)*) = self;
                [$(unsafe { $gtup.get_raw() as _ },)*]
            }
        }
    };

    (@arrays $($len:expr)*) => {$(
        unsafe impl<'a, G> GroupableGeometry<'a> for [G; $len] where G: GeometryType {
            type List = [*mut ID2D1Geometry; $len];
            #[inline]
            fn raw_geometry_list(&'a self) -> Self::List {
                let mut data = [ptr::null_mut(); $len];
                for (src, dst) in self.iter().zip(data.iter_mut()) {
                    *dst = unsafe { src.get_raw() as _ };
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
