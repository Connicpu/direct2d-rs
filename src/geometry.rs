use error::Error;
use stroke_style::StrokeStyle;
use factory::Factory;
use helpers::{ret_obj, GetRaw};
use math;

use std::{mem, ptr};
use std::marker::PhantomData;

use winapi::shared::minwindef::*;
use winapi::shared::winerror::*;
use winapi::um::d2d1::*;
use winapi::um::d2d1_1::*;
use winapi::um::d2dbasetypes::*;
use wio::com::ComPtr;

geometry_types! {
    pub struct GenericGeometry(ID2D1Geometry);

    /// Represents a rectangle which can be used anywhere Geometry is needed
    pub struct Rectangle(ID2D1RectangleGeometry);

    /// Represents a rounded rectangle which can be used anywhere Geometry is needed
    pub struct RoundedRectangle(ID2D1RoundedRectangleGeometry);

    /// Represents an ellipse which can be used anywhere Geometry is needed
    pub struct Ellipse(ID2D1EllipseGeometry);

    /// Represents multiple geometries combined into a single item
    pub struct Group(ID2D1GeometryGroup);

    /// Another piece of geometry which has had a transformation applied to it
    pub struct Transformed(ID2D1TransformedGeometry);

    /// Custom-shaped geometry made of lines and curves
    pub struct Path(ID2D1PathGeometry1);
}

impl GenericGeometry {
    pub fn as_rectangle(&self) -> Option<Rectangle> {
        Some(Rectangle {
            ptr: self.ptr.cast().ok()?,
        })
    }

    pub fn as_rounded_rectangle(&self) -> Option<RoundedRectangle> {
        Some(RoundedRectangle {
            ptr: self.ptr.cast().ok()?,
        })
    }

    pub fn as_ellipse(&self) -> Option<Ellipse> {
        Some(Ellipse {
            ptr: self.ptr.cast().ok()?,
        })
    }

    pub fn as_group(&self) -> Option<Group> {
        Some(Group {
            ptr: self.ptr.cast().ok()?,
        })
    }

    pub fn as_transformed(&self) -> Option<Transformed> {
        Some(Transformed {
            ptr: self.ptr.cast().ok()?,
        })
    }

    pub fn as_path(&self) -> Option<Path> {
        Some(Path {
            ptr: self.ptr.cast().ok()?,
        })
    }
}

impl Rectangle {
    pub fn get_rect(&self) -> math::RectF {
        unsafe {
            let mut rect: D2D1_RECT_F = mem::uninitialized();
            self.ptr.GetRect(&mut rect);
            math::RectF(rect)
        }
    }
}

impl RoundedRectangle {
    pub fn get_rounded_rect(&self) -> math::RoundedRect {
        unsafe {
            let mut rect: D2D1_ROUNDED_RECT = mem::uninitialized();
            self.ptr.GetRoundedRect(&mut rect);
            math::RoundedRect(rect)
        }
    }
}

impl Ellipse {
    pub fn get_ellipse(&self) -> math::Ellipse {
        unsafe {
            let mut ellipse: D2D1_ELLIPSE = mem::uninitialized();
            self.ptr.GetEllipse(&mut ellipse);
            math::Ellipse(ellipse)
        }
    }
}

impl Group {
    pub fn get_fill_mode(&self) -> Result<FillMode, Error> {
        unsafe { FillMode::from_raw(self.ptr.GetFillMode()) }
    }

    pub fn get_source_geometry_count(&self) -> u32 {
        unsafe { self.ptr.GetSourceGeometryCount() }
    }

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

impl Transformed {
    pub fn get_source_geometry(&self) -> GenericGeometry {
        unsafe {
            let mut ptr: *mut ID2D1Geometry = ptr::null_mut();
            self.ptr.GetSourceGeometry(&mut ptr);
            GenericGeometry {
                ptr: ComPtr::from_raw(ptr),
            }
        }
    }

    pub fn get_transform(&self) -> math::Matrix3x2F {
        unsafe {
            let mut matrix: D2D1_MATRIX_3X2_F = mem::uninitialized();
            self.ptr.GetTransform(&mut matrix);
            math::Matrix3x2F(matrix)
        }
    }
}

impl Path {
    pub fn open<'a>(&'a mut self) -> Result<GeometryBuilder<'a>, Error> {
        let mut ptr: *mut ID2D1GeometrySink = ptr::null_mut();
        unsafe {
            let result = self.ptr.Open(&mut ptr);
            if SUCCEEDED(result) {
                Ok(GeometryBuilder {
                    sink: ComPtr::from_raw(ptr),
                    phantom: PhantomData,
                })
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn get_segment_count(&self) -> Result<u32, Error> {
        unsafe {
            let mut count = 0;
            let result = self.ptr.GetSegmentCount(&mut count);
            if SUCCEEDED(result) {
                Ok(count)
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn get_figure_count(&self) -> Result<u32, Error> {
        unsafe {
            let mut count = 0;
            let result = self.ptr.GetFigureCount(&mut count);
            if SUCCEEDED(result) {
                Ok(count)
            } else {
                Err(From::from(result))
            }
        }
    }
}

/// Interface for building Path geometry
pub struct GeometryBuilder<'a> {
    sink: ComPtr<ID2D1GeometrySink>,
    phantom: PhantomData<&'a mut Path>,
}

impl<'a> GeometryBuilder<'a> {
    pub fn fill_mode(self, fill_mode: FillMode) -> Self {
        unsafe { self.sink.SetFillMode(fill_mode as u32) };
        self
    }

    pub fn set_segment_flags(self, flags: PathSegment) -> Self {
        unsafe { self.sink.SetSegmentFlags(flags as u32) };
        self
    }

    pub fn begin_figure<P: Into<math::Point2F>>(
        self,
        start: P,
        begin: FigureBegin,
        end: FigureEnd,
    ) -> FigureBuilder<'a> {
        unsafe {
            self.sink.BeginFigure(start.into().0, begin as u32);
        }
        FigureBuilder {
            builder: self,
            end: (end as u32),
        }
    }

    pub fn close(self) {
        // Drop self
    }
}

impl<'a> Drop for GeometryBuilder<'a> {
    fn drop(&mut self) {
        unsafe {
            let result = self.sink.Close();
            assert!(SUCCEEDED(result));
        }
    }
}

pub struct FigureBuilder<'a> {
    builder: GeometryBuilder<'a>,
    end: D2D1_FIGURE_END,
}

impl<'a> FigureBuilder<'a> {
    pub fn end(self) -> GeometryBuilder<'a> {
        unsafe {
            self.builder.sink.EndFigure(self.end);

            // Move builder out of self without invoking drop.
            let builder = ptr::read(&self.builder);
            mem::forget(self);
            builder
        }
    }

    pub fn add_line<P: Into<math::Point2F>>(self, point: P) -> Self {
        unsafe { self.builder.sink.AddLine(point.into().0) };
        self
    }

    pub fn add_lines(self, points: &[math::Point2F]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddLines(points.as_ptr() as *const _, points.len() as u32)
        };
        self
    }

    pub fn add_bezier(self, bezier: &math::BezierSegment) -> Self {
        unsafe { self.builder.sink.AddBezier(&bezier.0) };
        self
    }

    pub fn add_beziers(self, beziers: &[math::BezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    pub fn add_quadratic_bezier(self, bezier: &math::QuadBezierSegment) -> Self {
        unsafe { self.builder.sink.AddQuadraticBezier(&bezier.0) };
        self
    }

    pub fn add_quadratic_beziers(self, beziers: &[math::QuadBezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddQuadraticBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    pub fn add_arc(self, arc: &math::ArcSegment) -> Self {
        unsafe { self.builder.sink.AddArc(&arc.0) };
        self
    }
}

impl<'a> Drop for FigureBuilder<'a> {
    fn drop(&mut self) {
        unsafe {
            self.builder.sink.EndFigure(self.end);
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
    pub fn from_raw(value: D2D1_FILL_MODE) -> Result<FillMode, Error> {
        use self::FillMode::*;
        match value {
            D2D1_FILL_MODE_ALTERNATE => Ok(Alternate),
            D2D1_FILL_MODE_WINDING => Ok(Winding),
            _ => Err(Error::UnknownEnumValue),
        }
    }
}

pub enum FigureBegin {
    Filled = 0,
    Hollow = 1,
}

pub enum FigureEnd {
    Open = 0,
    Closed = 1,
}

pub enum PathSegment {
    None = 0,
    ForceUnstroked = 1,
    ForceRoundLineJoin = 2,
}

pub trait Geometry {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry;

    fn get_factory(&self) -> Factory {
        unsafe {
            let ptr = self.get_ptr();
            let mut factory: *mut ID2D1Factory = ptr::null_mut();
            (*ptr).GetFactory(&mut factory);

            Factory::from_ptr(ComPtr::from_raw(factory).cast().unwrap())
        }
    }

    fn to_generic(&self) -> GenericGeometry {
        let ptr = unsafe { ComPtr::from_raw(self.get_ptr()) };
        mem::forget(ptr.clone());
        GenericGeometry { ptr }
    }

    /// Retrieve the bounds of the geometry, with an optional applied transform.
    ///
    /// **NOTE:** I'm not sure if this will ever return None, but the API has an
    /// error code so it could. The MSDN documentation is very vague on this.
    fn get_bounds(&self, world_transform: Option<&math::Matrix3x2F>) -> Result<math::RectF, Error> {
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
        &self,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&math::Matrix3x2F>,
    ) -> Result<math::RectF, Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_ptr() as *mut _,
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
        &self,
        point: math::Point2F,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
        world_transform: Option<&math::Matrix3x2F>,
    ) -> Result<bool, Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };
            let stroke_style = match stroke_style {
                Some(stroke) => stroke.get_ptr() as *mut _,
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
        &self,
        point: math::Point2F,
        world_transform: Option<&math::Matrix3x2F>,
    ) -> Result<bool, Error> {
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
        &self,
        input: &T,
        input_transform: Option<&math::Matrix3x2F>,
    ) -> Result<GeometryRelation, Error> {
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
                    _ => Err(Error::UnknownEnumValue),
                }
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Computes the area of the geometry.
    fn compute_area(&self, world_transform: Option<&math::Matrix3x2F>) -> Result<f32, Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };

            let mut area = 0.0;
            let result = (*ptr).ComputeArea(matrix, D2D1_DEFAULT_FLATTENING_TOLERANCE, &mut area);

            if SUCCEEDED(result) {
                Ok(area)
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Computes the length of the geometry.
    fn compute_length(&self, world_transform: Option<&math::Matrix3x2F>) -> Result<f32, Error> {
        unsafe {
            let ptr = self.get_ptr();
            let matrix = match world_transform {
                Some(mat) => &mat.0 as *const _,
                None => ptr::null(),
            };

            let mut length = 0.0;
            let result =
                (*ptr).ComputeLength(matrix, D2D1_DEFAULT_FLATTENING_TOLERANCE, &mut length);

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
        world_transform: Option<&math::Matrix3x2F>,
    ) -> Result<(math::Point2F, math::Vector2F), Error> {
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

    fn transformed(&self, transform: &math::Matrix3x2F) -> Result<Transformed, Error> {
        let factory = self.get_factory();
        unsafe {
            let raw_factory = factory.get_raw();
            let mut ptr: *mut ID2D1TransformedGeometry = ptr::null_mut();
            let hr =
                (*raw_factory).CreateTransformedGeometry(self.get_ptr(), &transform.0, &mut ptr);

            ret_obj(hr, ptr)
        }
    }
}

impl<'a, T: Geometry> Geometry for &'a T {
    unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
        T::get_ptr(*self)
    }
}
