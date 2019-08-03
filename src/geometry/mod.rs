use crate::enums::*;
use crate::resource::IResource;
use crate::stroke_style::StrokeStyle;

use std::mem::MaybeUninit;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::{Matrix3x2f, Point2f, Rectf, Vector2f};
use winapi::shared::minwindef::*;
use winapi::shared::winerror::*;
use winapi::um::d2d1::*;
use wio::com::ComPtr;

pub use self::ellipse::EllipseGeometry;
pub use self::group::GroupGeometry;
pub use self::path::PathGeometry;
pub use self::rectangle::RectangleGeometry;
pub use self::rounded_rectangle::RoundedRectangleGeometry;
pub use self::transformed::TransformedGeometry;

pub mod ellipse;
pub mod group;
pub mod path;
pub mod rectangle;
pub mod rounded_rectangle;
pub mod transformed;

#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct Geometry {
    ptr: ComPtr<ID2D1Geometry>,
}

pub unsafe trait IGeometry: IResource {
    /// Retrieve the bounds of the geometry, with an optional applied transform.
    ///
    /// **NOTE:** I'm not sure if this will ever return None, but the API has an
    /// error code so it could. The MSDN documentation is very vague on this.
    fn bounds(&self, world_transform: Option<&Matrix3x2f>) -> Result<Rectf, Error> {
        unsafe {
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => std::ptr::null(),
            };

            let mut rect = MaybeUninit::uninit();
            let result = self.raw_geom().GetBounds(matrix, rect.as_mut_ptr());
            if SUCCEEDED(result) {
                Ok(rect.assume_init().into())
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Get the bounds of the corresponding geometry after it has been widened or have
    /// an optional pen style applied.
    fn widened_bounds(
        &self,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&Matrix3x2f>,
    ) -> Result<Rectf, Error> {
        unsafe {
            let matrix = match world_transform {
                Some(mat) => &mat as *const _ as *const _,
                None => std::ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            let mut rect = MaybeUninit::uninit();
            let result = self.raw_geom().GetWidenedBounds(
                stroke_width,
                stroke_style,
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
                rect.as_mut_ptr(),
            );

            if SUCCEEDED(result) {
                Ok(rect.assume_init().into())
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Checks to see whether the corresponding penned and widened geometry contains the
    /// given point.
    fn stroke_contains_point(
        &self,
        point: Point2f,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&Matrix3x2f>,
    ) -> Result<bool, Error> {
        unsafe {
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => std::ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_raw() as *mut _,
                None => std::ptr::null_mut(),
            };

            let mut contains: BOOL = 0;
            let result = self.raw_geom().StrokeContainsPoint(
                point.into(),
                stroke_width,
                stroke_style,
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
                &mut contains,
            );

            if SUCCEEDED(result) {
                Ok(contains != 0)
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Test whether the given fill of this geometry would contain this point.
    fn fill_contains_point(
        &self,
        point: Point2f,
        world_transform: Option<&Matrix3x2f>,
    ) -> Result<bool, Error> {
        unsafe {
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => std::ptr::null(),
            };

            let mut contains: BOOL = 0;
            let result = self.raw_geom().FillContainsPoint(
                point.into(),
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
                &mut contains,
            );

            if SUCCEEDED(result) {
                Ok(contains != 0)
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Compare how one geometry intersects or contains another geometry.
    fn compare_with_geometry(
        &self,
        input: &dyn IGeometry,
        input_transform: Option<&Matrix3x2f>,
    ) -> Result<UncheckedEnum<GeometryRelation>, Error> {
        unsafe {
            let self_ptr = self.raw_geom();
            let input_ptr = input.raw_geom();

            let matrix = match input_transform {
                Some(mat) => mat as *const _ as *const _,
                None => std::ptr::null(),
            };

            let mut relation: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION_UNKNOWN;
            let result = self_ptr.CompareWithGeometry(
                input_ptr as *const _ as _,
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
                &mut relation,
            );

            if SUCCEEDED(result) {
                Ok(relation.into())
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Computes the area of the geometry.
    fn compute_area(&self, world_transform: Option<&Matrix3x2f>) -> Result<f32, Error> {
        unsafe {
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => std::ptr::null(),
            };

            let mut area = 0.0;
            let tolerance = D2D1_DEFAULT_FLATTENING_TOLERANCE;
            let result = self.raw_geom().ComputeArea(matrix, tolerance, &mut area);

            if SUCCEEDED(result) {
                Ok(area)
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Computes the length of the geometry.
    fn compute_length(&self, world_transform: Option<&Matrix3x2f>) -> Result<f32, Error> {
        unsafe {
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => std::ptr::null(),
            };

            let mut length = 0.0;
            let tolerance = D2D1_DEFAULT_FLATTENING_TOLERANCE;
            let result = self
                .raw_geom()
                .ComputeLength(matrix, tolerance, &mut length);

            if SUCCEEDED(result) {
                Ok(length)
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Computes the point and tangent at a given distance along the path.
    fn compute_point_at_length(
        &self,
        length: f32,
        world_transform: Option<&Matrix3x2f>,
    ) -> Result<(Point2f, Vector2f), Error> {
        unsafe {
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => std::ptr::null(),
            };

            let mut point = MaybeUninit::uninit();
            let mut tangent = MaybeUninit::uninit();
            let result = self.raw_geom().ComputePointAtLength(
                length,
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
                point.as_mut_ptr(),
                tangent.as_mut_ptr(),
            );

            if SUCCEEDED(result) {
                let tangent = tangent.assume_init();
                Ok((point.assume_init().into(), [tangent.x, tangent.y].into()))
            } else {
                Err(From::from(result))
            }
        }
    }

    fn transformed(&self, transform: &Matrix3x2f) -> Result<TransformedGeometry, Error> {
        let factory = self.factory();
        unsafe {
            let raw_factory = factory.get_raw();
            let mut geometry = std::ptr::null_mut();
            let hr = (*raw_factory).CreateTransformedGeometry(
                self.raw_geom() as *const _ as *mut _,
                transform as *const _ as *const _,
                &mut geometry,
            );

            if SUCCEEDED(hr) {
                Ok(TransformedGeometry::from_raw(geometry))
            } else {
                Err(hr.into())
            }
        }
    }

    unsafe fn raw_geom(&self) -> &ID2D1Geometry;
}

unsafe impl IResource for Geometry {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}

unsafe impl IGeometry for Geometry {
    unsafe fn raw_geom(&self) -> &ID2D1Geometry {
        &self.ptr
    }
}

pub unsafe trait GeometryType: ComWrapper + Clone {
    fn to_generic(&self) -> Geometry
    where
        Self: Sized,
    {
        unsafe { Geometry::from_ptr(self.clone().into_ptr().cast().unwrap()) }
    }
}

unsafe impl GeometryType for Geometry {}
