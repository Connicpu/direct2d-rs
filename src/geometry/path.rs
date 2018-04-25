use enums::{FigureBegin, FigureEnd, FillMode, PathSegment};
use error::D2DResult;
use factory::Factory;
use math;

use std::marker::PhantomData;
use std::{mem, ptr};

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{D2D1_FIGURE_END, ID2D1GeometrySink};
use winapi::um::d2d1_1::ID2D1PathGeometry1;
use wio::com::ComPtr;

/// Custom-shaped geometry made of lines and curves
#[repr(C)]
#[derive(Clone)]
pub struct Path {
    ptr: ComPtr<ID2D1PathGeometry1>,
}

impl Path {
    #[inline]
    pub fn create(factory: &Factory) -> D2DResult<Path> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = (*factory.get_raw()).CreatePathGeometry(&mut ptr);
            if SUCCEEDED(hr) {
                Ok(Path::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn open<'a>(&'a mut self) -> D2DResult<GeometryBuilder<'a>> {
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

    #[inline]
    pub fn get_segment_count(&self) -> D2DResult<u32> {
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

    #[inline]
    pub fn get_figure_count(&self) -> D2DResult<u32> {
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

geometry_type!(Path: ID2D1PathGeometry1);

/// Interface for building Path geometry
pub struct GeometryBuilder<'a> {
    sink: ComPtr<ID2D1GeometrySink>,
    phantom: PhantomData<&'a mut Path>,
}

impl<'a> GeometryBuilder<'a> {
    #[inline]
    pub fn fill_mode(self, fill_mode: FillMode) -> Self {
        unsafe { self.sink.SetFillMode(fill_mode as u32) };
        self
    }

    #[inline]
    pub fn set_segment_flags(self, flags: PathSegment) -> Self {
        unsafe { self.sink.SetSegmentFlags(flags as u32) };
        self
    }

    #[inline]
    pub fn begin_figure<P>(self, start: P, begin: FigureBegin, end: FigureEnd) -> FigureBuilder<'a>
    where
        P: Into<math::Point2F>,
    {
        unsafe {
            self.sink.BeginFigure(start.into().0, begin as u32);
        }
        FigureBuilder {
            builder: self,
            end: (end as u32),
        }
    }

    #[inline]
    pub fn with_figure<P, F>(self, start: P, begin: FigureBegin, end: FigureEnd, f: F) -> Self
    where
        P: Into<math::Point2F>,
        F: FnOnce(FigureBuilder<'a>) -> FigureBuilder<'a>,
    {
        f(self.begin_figure(start, begin, end)).end()
    }

    #[inline]
    pub fn copy_from(self, path: &Path) -> D2DResult<Self> {
        unsafe {
            let hr = path.ptr.Stream(self.sink.as_raw());
            if SUCCEEDED(hr) {
                Ok(self)
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn close(self) -> D2DResult<()> {
        unsafe {
            let hr = self.sink.Close();
            mem::forget(self);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }
}

impl<'a> Drop for GeometryBuilder<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let result = self.sink.Close();
            assert!(
                SUCCEEDED(result),
                "Failed to close dropped GeometryBuilder. You should call \
                 .close() manually if you would like to handle this error"
            );
        }
    }
}

pub struct FigureBuilder<'a> {
    builder: GeometryBuilder<'a>,
    end: D2D1_FIGURE_END,
}

impl<'a> FigureBuilder<'a> {
    #[inline]
    pub fn end(self) -> GeometryBuilder<'a> {
        unsafe {
            self.builder.sink.EndFigure(self.end);

            // Move builder out of self without invoking drop.
            let builder = ptr::read(&self.builder);
            mem::forget(self);
            builder
        }
    }

    #[inline]
    pub fn add_line<P: Into<math::Point2F>>(self, point: P) -> Self {
        unsafe { self.builder.sink.AddLine(point.into().0) };
        self
    }

    #[inline]
    pub fn add_lines(self, points: &[math::Point2F]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddLines(points.as_ptr() as *const _, points.len() as u32)
        };
        self
    }

    #[inline]
    pub fn add_bezier(self, bezier: &math::BezierSegment) -> Self {
        unsafe { self.builder.sink.AddBezier(&bezier.0) };
        self
    }

    #[inline]
    pub fn add_beziers(self, beziers: &[math::BezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    #[inline]
    pub fn add_quadratic_bezier(self, bezier: &math::QuadBezierSegment) -> Self {
        unsafe { self.builder.sink.AddQuadraticBezier(&bezier.0) };
        self
    }

    #[inline]
    pub fn add_quadratic_beziers(self, beziers: &[math::QuadBezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddQuadraticBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    #[inline]
    pub fn add_arc(self, arc: &math::ArcSegment) -> Self {
        unsafe { self.builder.sink.AddArc(&arc.0) };
        self
    }
}

impl<'a> Drop for FigureBuilder<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.builder.sink.EndFigure(self.end);
        }
    }
}
