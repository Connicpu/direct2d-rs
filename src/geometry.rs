use std::{mem, ptr};
use winapi::*;
use comptr::ComPtr;
use error::D2D1Error;
use stroke_style::StrokeStyle;
use factory::Factory;
use helpers::{GetRaw, FromRaw};
use math;

pub trait Geometry {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry;
    
    fn get_factory(&self) -> Factory {
        unsafe {
            let ptr = self.get_ptr();
            let mut factory = ComPtr::<ID2D1Factory>::new();
            (*ptr).GetFactory(factory.raw_addr());
            
            Factory::from_ptr(factory)
        }
    }
    
    fn to_generic(&self) -> GenericGeometry {
        GenericGeometry {
            geom: unsafe { ComPtr::from_existing(self.get_ptr()) },
        }
    }
    
    /// Retrieve the bounds of the geometry, with an optional applied transform.
    /// 
    /// **NOTE:** I'm not sure if this will ever return None, but the API has an
    /// error code so it could. The MSDN documentation is very vague on this.
    fn get_bounds(&self, world_transform: Option<&math::Matrix3x2F>) -> Result<math::RectF, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
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
        world_transform: Option<&math::Matrix3x2F>
    ) -> Result<math::RectF, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
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
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
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
        world_transform: Option<&math::Matrix3x2F>
    ) -> Result<bool, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
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
        &self, point: math::Point2F, world_transform: Option<&math::Matrix3x2F>
    )-> Result<bool, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut contains: BOOL = 0;
            let result = (*ptr).FillContainsPoint(
                point.0,
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
    fn compare_with_geometry<T: Geometry>(
        &self, input: &T, input_transform: Option<&math::Matrix3x2F>
    ) -> Result<GeometryRelation, D2D1Error> {
        unsafe {
            let self_ptr = self.get_ptr();
            let input_ptr = input.get_ptr();
            
            let matrix = match input_transform {
                Some(mat) => &mat.0 as *const _,
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
        &self, world_transform: Option<&math::Matrix3x2F>
    ) -> Result<f32, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut area = 0.0;
            let result = (*ptr).ComputeArea(
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
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
        &self, world_transform: Option<&math::Matrix3x2F>
    ) -> Result<f32, D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            
            let mut length = 0.0;
            let result = (*ptr).ComputeLength(
                matrix,
                D2D1_DEFAULT_FLATTENING_TOLERANCE,
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
        &self, length: f32, world_transform: Option<&math::Matrix3x2F>
    ) -> Result<(math::Point2F, math::Vector2F), D2D1Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
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
    
    fn transformed(&self, transform: &math::Matrix3x2F) -> Result<Transformed, D2D1Error> {
        let factory = self.get_factory();
        unsafe {
            let raw_factory = factory.get_raw();
            let mut transformed: ComPtr<ID2D1TransformedGeometry> = ComPtr::new();
            let result = (*raw_factory).CreateTransformedGeometry(
                self.get_ptr(),
                &transform.0,
                transformed.raw_addr(),
            );
            
            if SUCCEEDED(result) {
                Ok(Transformed { geom: transformed })
            } else {
                Err(From::from(result))
            }
        }
    }
}

impl<'a, T: Geometry> Geometry for &'a T {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        T::get_ptr(*self)
    }
}

/// Represents generic geometry of an unknown type
#[derive(Clone, PartialEq)]
pub struct GenericGeometry {
    geom: ComPtr<ID2D1Geometry>,
}

impl GenericGeometry {
    pub fn as_rectangle(&self) -> Option<Rectangle> {
        match self.geom.query_interface::<ID2D1RectangleGeometry>() {
            Ok(ptr) => Some(Rectangle { geom: ptr }),
            Err(_) => None,
        }
    }
}

impl Geometry for GenericGeometry {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        &mut *(&mut *self.geom.raw_value())
    }
}

impl FromRaw for GenericGeometry {
    type Raw = ID2D1Geometry;
    unsafe fn from_raw(raw: *mut ID2D1Geometry) -> Self {
        GenericGeometry {
            geom: ComPtr::from_existing(raw)
        }
    }
}

/// Represents a rectangle which can be used anywhere Geometry is needed
#[derive(Clone, PartialEq)]
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

impl FromRaw for Rectangle {
    type Raw = ID2D1RectangleGeometry;
    unsafe fn from_raw(raw: *mut ID2D1RectangleGeometry) -> Self {
        Rectangle {
            geom: ComPtr::from_existing(raw)
        }
    }
}

/// Represents a rounded rectangle which can be used anywhere Geometry is needed
#[derive(Clone, PartialEq)]
pub struct RoundedRectangle {
    geom: ComPtr<ID2D1RoundedRectangleGeometry>,
}

impl RoundedRectangle {
    pub fn get_rounded_rect(&self) -> math::RoundedRect {
        unsafe {
            let mut rect: D2D1_ROUNDED_RECT = mem::uninitialized();
            (*self.geom.raw_value()).GetRoundedRect(&mut rect);
            math::RoundedRect(rect)
        }
    }
}

impl Geometry for RoundedRectangle {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        &mut **(&mut *self.geom.raw_value())
    }
}

impl FromRaw for RoundedRectangle {
    type Raw = ID2D1RoundedRectangleGeometry;
    unsafe fn from_raw(raw: *mut ID2D1RoundedRectangleGeometry) -> Self {
        RoundedRectangle {
            geom: ComPtr::from_existing(raw)
        }
    }
}

/// Represents an ellipse which can be used anywhere Geometry is needed
#[derive(Clone, PartialEq)]
pub struct Ellipse {
    geom: ComPtr<ID2D1EllipseGeometry>,
}

impl Ellipse {
    pub fn get_ellipse(&self) -> math::Ellipse {
        unsafe {
            let mut ellipse: D2D1_ELLIPSE = mem::uninitialized();
            (*self.geom.raw_value()).GetEllipse(&mut ellipse);
            math::Ellipse(ellipse)
        }
    }
}

impl Geometry for Ellipse {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        &mut **(&mut *self.geom.raw_value())
    }
}

impl FromRaw for Ellipse {
    type Raw = ID2D1EllipseGeometry;
    unsafe fn from_raw(raw: *mut ID2D1EllipseGeometry) -> Self {
        Ellipse {
            geom: ComPtr::from_existing(raw)
        }
    }
}

/// Represents multiple geometries combined into a single item
#[derive(Clone, PartialEq)]
pub struct Group {
    geom: ComPtr<ID2D1GeometryGroup>,
}

impl Group {
    pub fn get_fill_mode(&self) -> Result<FillMode, D2D1Error> {
        unsafe {
            FillMode::from_raw((*self.geom.raw_value()).GetFillMode())
        }
    }
    
    pub fn get_source_geometry_count(&self) -> u32 {
        unsafe {
            (*self.geom.raw_value()).GetSourceGeometryCount()
        }
    }
    
    pub fn get_source_geometries(&self) -> Vec<GenericGeometry> {
        unsafe {
            let count = self.get_source_geometry_count();
            let mut data: Vec<GenericGeometry> = vec![GenericGeometry { geom: ComPtr::new() }; count as usize];
            (*self.geom.raw_value()).GetSourceGeometries(data.as_mut_ptr() as *mut _, count);
            data
        }
    }
}

impl Geometry for Group {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        &mut **(&mut *self.geom.raw_value())
    }
}

impl FromRaw for Group {
    type Raw = ID2D1GeometryGroup;
    unsafe fn from_raw(raw: *mut ID2D1GeometryGroup) -> Self {
        Group {
            geom: ComPtr::from_existing(raw)
        }
    }
}

/// Represents geometry which has had a transformation applied to it
#[derive(Clone, PartialEq)]
pub struct Transformed {
    geom: ComPtr<ID2D1TransformedGeometry>,
}

impl Transformed {
    pub fn get_source_geometry(&self) -> GenericGeometry {
        unsafe {
            let mut ptr: ComPtr<ID2D1Geometry> = ComPtr::new();
            (*self.geom.raw_value()).GetSourceGeometry(ptr.raw_addr());
            GenericGeometry { geom: ptr }
        }
    }
    
    pub fn get_transform(&self) -> math::Matrix3x2F {
        unsafe {
            let mut matrix: D2D1_MATRIX_3X2_F = mem::uninitialized();
            (*self.geom.raw_value()).GetTransform(&mut matrix);
            math::Matrix3x2F(matrix)
        }
    }
}

impl Geometry for Transformed {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        &mut **(&mut *self.geom.raw_value())
    }
}

impl FromRaw for Transformed {
    type Raw = ID2D1TransformedGeometry;
    unsafe fn from_raw(raw: *mut ID2D1TransformedGeometry) -> Self {
        Transformed {
            geom: ComPtr::from_existing(raw)
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

pub enum FillMode {
    Alternate = 0,
    Winding = 1,
}

impl FillMode {
    pub fn from_raw(value: D2D1_FILL_MODE) -> Result<FillMode, D2D1Error> {
        use self::FillMode::*;
        match value {
            D2D1_FILL_MODE_ALTERNATE => Ok(Alternate),
            D2D1_FILL_MODE_WINDING => Ok(Winding),
            _ => Err(D2D1Error::UnknownEnumValue)
        }
    }
}

