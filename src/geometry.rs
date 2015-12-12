use std::{mem, ptr};
use winapi::*;
use comptr::ComPtr;
use error::D2D1Error;
use stroke_style::StrokeStyle;
use math;

pub trait Geometry {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry;
    
    /// Retrieve the bounds of the geometry, with an optional applied transform.
    /// 
    /// **NOTE:** I'm not sure if this will ever return None, but the API has an
    /// error code so it could. The MSDN documentation is very vague on this.
    fn get_bounds(&self, world_transform: Option<&math::Matrix3x2F>) -> Result<math::RectF, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            assert!(!ptr.is_null());
            
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut rect: D2D1_RECT_F = mem::uninitialized();
            let result = (*ptr).GetBounds(matrix, &mut rect);
            if SUCCEEDED(result) {
                Ok(math::RectF(rect))
            } else {
                Err(From::from(result))
            }
        }
    }
    
    /// Get the bounds of the corresponding geometry after it has been widened or have
    /// an optional pen style applied.
    fn get_widened_bounds(
        &self, stroke_width: f32, stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&math::Matrix3x2F>, flattening_tolerance: f32
    ) -> Result<math::RectF, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            assert!(!ptr.is_null());
            
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_ptr(),
                None => ptr::null_mut(),
            };
            
            let mut rect: D2D1_RECT_F = mem::uninitialized();
            let result = (*ptr).GetWidenedBounds(
                stroke_width,
                stroke_style,
                matrix,
                flattening_tolerance,
                &mut rect,
            );
            
            if SUCCEEDED(result) {
                Ok(math::RectF(rect))
            } else {
                Err(From::from(result))
            }
        }
    }
    
    /// Checks to see whether the corresponding penned and widened geometry contains the
    /// given point.
    fn stroke_contains_point(
        &self, point: math::Point2F, stroke_width: f32, stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&math::Matrix3x2F>, flattening_tolerance: f32
    ) -> Result<bool, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            assert!(!ptr.is_null());
            
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_ptr(),
                None => ptr::null_mut(),
            };
            
            let mut contains: BOOL = 0;
            let result = (*ptr).StrokeContainsPoint(
                point.0,
                stroke_width,
                stroke_style,
                matrix,
                flattening_tolerance,
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
        &self, point: math::Point2F, world_transform: Option<&math::Matrix3x2F>,
        flattening_tolerance: f32
    )-> Result<bool, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            assert!(!ptr.is_null());
            
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut contains: BOOL = 0;
            let result = (*ptr).FillContainsPoint(
                point.0,
                matrix,
                flattening_tolerance,
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
    fn compare_with_geometry<T: Geometry>(
        &self, input: &T, input_transform: Option<&math::Matrix3x2F>, flattening_tolerance: f32
    ) -> Result<GeometryRelation, D2D1Error> {
        unsafe {
            let self_ptr = self.get_ptr();
            assert!(!self_ptr.is_null());
            let input_ptr = input.get_ptr();
            assert!(!input_ptr.is_null());
            
            let matrix = match input_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut relation: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION_UNKNOWN;
            let result = (*self_ptr).CompareWithGeometry(
                input_ptr,
                matrix,
                flattening_tolerance,
                &mut relation,
            );
            
            if SUCCEEDED(result) {
                use self::GeometryRelation::*;
                match relation {
                    D2D1_GEOMETRY_RELATION_UNKNOWN => Ok(Unknown),
                    D2D1_GEOMETRY_RELATION_DISJOINT => Ok(Disjoint),
                    D2D1_GEOMETRY_RELATION_IS_CONTAINED => Ok(IsContained),
                    D2D1_GEOMETRY_RELATION_CONTAINS => Ok(Contains),
                    D2D1_GEOMETRY_RELATION_OVERLAP => Ok(Overlap),
                    _ => Err(D2D1Error::UnknownEnumValue),
                }
            } else {
                Err(From::from(result))
            }
        }
    }
    
    /// Computes the area of the geometry.
    fn compute_area(
        &self, world_transform: Option<&math::Matrix3x2F>, flattening_tolerance: f32
    ) -> Result<f32, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            assert!(!ptr.is_null());
            
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut area = 0.0;
            let result = (*ptr).ComputeArea(
                matrix,
                flattening_tolerance,
                &mut area,
            );
            
            if SUCCEEDED(result) {
                Ok(area)
            } else {
                Err(From::from(result))
            }
        }
    }
    
    /// Computes the length of the geometry.
    fn compute_length(
        &self, world_transform: Option<&math::Matrix3x2F>, flattening_tolerance: f32
    ) -> Result<f32, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            assert!(!ptr.is_null());
            
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut length = 0.0;
            let result = (*ptr).ComputeLength(
                matrix,
                flattening_tolerance,
                &mut length,
            );
            
            if SUCCEEDED(result) {
                Ok(length)
            } else {
                Err(From::from(result))
            }
        }
    }
    
    /// Computes the point and tangent at a given distance along the path.
    fn compute_point_at_length(
        &self, length: f32, world_transform: Option<&math::Matrix3x2F>, flattening_tolerance: f32
    ) -> Result<(math::Point2F, math::Vector2F), D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            assert!(!ptr.is_null());
            
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut point: D2D1_POINT_2F = mem::uninitialized();
            let mut tangent: D2D1_POINT_2F = mem::uninitialized();
            let result = (*ptr).ComputePointAtLength(
                length,
                matrix,
                flattening_tolerance,
                &mut point,
                &mut tangent,
            );
            
            if SUCCEEDED(result) {
                Ok((
                    math::Point2F(point),
                    math::Vector2F(D2D_VECTOR_2F {
                        x: tangent.x,
                        y: tangent.y,
                    }),
                ))
            } else {
                Err(From::from(result))
            }
        }
    }
}

pub enum GeometryRelation {
    Unknown = 0,
    Disjoint = 1,
    IsContained = 2,
    Contains = 3,
    Overlap = 4,
}

/// Represents a rectangle which can be used anywhere Geometry is needed
pub struct Rectangle {
    geom: ComPtr<ID2D1RectangleGeometry>,
}

impl Rectangle {
    pub fn get_rect(&self) -> math::RectF {
        unsafe {
            let mut rect: D2D1_RECT_F = mem::uninitialized();
            (*self.geom.raw_value()).GetRect(&mut rect);
            math::RectF(rect)
        }
    }
}

impl Geometry for Rectangle {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        &mut **(&mut *self.geom.raw_value())
    }
}

/// Represents a rounded rectangle which can be used anywhere Geometry is needed
pub struct RoundedRectangle {
    geom: ComPtr<ID2D1RoundedRectangleGeometry>,
}

impl Geometry for RoundedRectangle {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        &mut **(&mut *self.geom.raw_value())
    }
}

