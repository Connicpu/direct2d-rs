use crate::enums::*;
use crate::error::D2DResult;
use crate::stroke_style::StrokeStyle;
use crate::resource::Resource;

use std::{mem, ptr};

use dcommon::helpers::deref_com_wrapper;
use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use math2d::{Matrix3x2f, Point2f, Rectf, Vector2f};
use winapi::shared::minwindef::*;
use winapi::shared::winerror::*;
use winapi::um::d2d1::*;
use winapi::um::d2dbasetypes::{D2D1_POINT_2F, D2D1_RECT_F};
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

impl Geometry {
    #[inline]
    /// Retrieve the bounds of the geometry, with an optional applied transform.
    ///
    /// **NOTE:** I'm not sure if this will ever return None, but the API has an
    /// error code so it could. The MSDN documentation is very vague on this.
    pub fn bounds(&self, world_transform: Option<&Matrix3x2f>) -> D2DResult<Rectf> {
        unsafe {
            let ptr = self.get_raw();
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => ptr::null(),
            };

            let mut rect: D2D1_RECT_F = mem::uninitialized();
            let result = (*ptr).GetBounds(matrix, &mut rect);
            if SUCCEEDED(result) {
                Ok(rect.into())
            } else {
                Err(From::from(result))
            }
        }
    }

    #[inline]
    /// Get the bounds of the corresponding geometry after it has been widened or have
    /// an optional pen style applied.
    pub fn widened_bounds(
        &self,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&Matrix3x2f>,
    ) -> D2DResult<Rectf> {
        unsafe {
            let ptr = self.get_raw();
            let matrix = match world_transform {
                Some(mat) => &mat as *const _ as *const _,
                None => ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_raw() as *mut _,
                None => ptr::null_mut(),
            };

            let mut rect: D2D1_RECT_F = mem::uninitialized();
            let result = (*ptr).GetWidenedBounds(
                stroke_width,
                stroke_style,
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
                &mut rect,
            );

            if SUCCEEDED(result) {
                Ok(rect.into())
            } else {
                Err(From::from(result))
            }
        }
    }

    #[inline]
    /// Checks to see whether the corresponding penned and widened geometry contains the
    /// given point.
    pub fn stroke_contains_point(
        &self,
        point: Point2f,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&Matrix3x2f>,
    ) -> D2DResult<bool> {
        unsafe {
            let ptr = self.get_raw();
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_raw() as *mut _,
                None => ptr::null_mut(),
            };

            let mut contains: BOOL = 0;
            let result = (*ptr).StrokeContainsPoint(
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

    #[inline]
    /// Test whether the given fill of this geometry would contain this point.
    pub fn fill_contains_point(
        &self,
        point: Point2f,
        world_transform: Option<&Matrix3x2f>,
    ) -> D2DResult<bool> {
        unsafe {
            let ptr = self.get_raw();
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => ptr::null(),
            };

            let mut contains: BOOL = 0;
            let result = (*ptr).FillContainsPoint(
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

    #[inline]
    /// Compare how one geometry intersects or contains another geometry.
    pub fn compare_with_geometry(
        &self,
        input: &Geometry,
        input_transform: Option<&Matrix3x2f>,
    ) -> D2DResult<UncheckedEnum<GeometryRelation>> {
        unsafe {
            let self_ptr = self.get_raw();
            let input_ptr = input.get_raw();

            let matrix = match input_transform {
                Some(mat) => mat as *const _ as *const _,
                None => ptr::null(),
            };

            let mut relation: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION_UNKNOWN;
            let result = (*self_ptr).CompareWithGeometry(
                input_ptr,
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

    #[inline]
    /// Computes the area of the geometry.
    pub fn compute_area(&self, world_transform: Option<&Matrix3x2f>) -> D2DResult<f32> {
        unsafe {
            let ptr = self.get_raw();
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => ptr::null(),
            };

            let mut area = 0.0;
            let tolerance = D2D1_DEFAULT_FLATTENING_TOLERANCE;
            let result = (*ptr).ComputeArea(matrix, tolerance, &mut area);

            if SUCCEEDED(result) {
                Ok(area)
            } else {
                Err(From::from(result))
            }
        }
    }

    #[inline]
    /// Computes the length of the geometry.
    pub fn compute_length(&self, world_transform: Option<&Matrix3x2f>) -> D2DResult<f32> {
        unsafe {
            let ptr = self.get_raw();
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => ptr::null(),
            };

            let mut length = 0.0;
            let tolerance = D2D1_DEFAULT_FLATTENING_TOLERANCE;
            let result = (*ptr).ComputeLength(matrix, tolerance, &mut length);

            if SUCCEEDED(result) {
                Ok(length)
            } else {
                Err(From::from(result))
            }
        }
    }

    #[inline]
    /// Computes the point and tangent at a given distance along the path.
    pub fn compute_point_at_length(
        &self,
        length: f32,
        world_transform: Option<&Matrix3x2f>,
    ) -> D2DResult<(Point2f, Vector2f)> {
        unsafe {
            let ptr = self.get_raw();
            let matrix = match world_transform {
                Some(mat) => mat as *const _ as *const _,
                None => ptr::null(),
            };

            let mut point: D2D1_POINT_2F = mem::uninitialized();
            let mut tangent: D2D1_POINT_2F = mem::uninitialized();
            let result = (*ptr).ComputePointAtLength(
                length,
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
                &mut point,
                &mut tangent,
            );

            if SUCCEEDED(result) {
                Ok((point.into(), [tangent.x, tangent.y].into()))
            } else {
                Err(From::from(result))
            }
        }
    }

    #[inline]
    pub fn transformed(&self, transform: &Matrix3x2f) -> D2DResult<TransformedGeometry> {
        let factory = self.factory();
        unsafe {
            let raw_factory = factory.get_raw();
            let mut geometry = ptr::null_mut();
            let hr = (*raw_factory).CreateTransformedGeometry(
                self.get_raw(),
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
}

impl std::ops::Deref for Geometry {
    type Target = Resource;
    fn deref(&self) -> &Resource {
        unsafe { deref_com_wrapper(self) }
    }
}

pub unsafe trait GeometryType: ComWrapper + Clone {
    fn to_generic(&self) -> Geometry where Self: Sized {
        unsafe { Geometry::from_ptr(self.clone().into_ptr().cast().unwrap()) }
    }
}

unsafe impl GeometryType for Geometry {}
