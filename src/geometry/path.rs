use crate::enums::{FigureBegin, FigureEnd, FillMode, PathSegment};
use crate::error::D2DResult;
use crate::factory::Factory;
use math2d::{ArcSegment, BezierSegment, Point2f, QuadBezierSegment};

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1GeometrySink, D2D1_FIGURE_END};
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
    pub fn create(factory: &Factory) -> D2DResult<PathBuilder> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreatePathGeometry(&mut ptr);
            if SUCCEEDED(hr) {
                let path = Path::from_raw(ptr);

                let mut ptr = std::ptr::null_mut();
                let hr = path.ptr.Open(&mut ptr);

                if SUCCEEDED(hr) {
                    Ok(PathBuilder {
                        path: path,
                        sink: ComPtr::from_raw(ptr),
                    })
                } else {
                    Err(hr.into())
                }
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn segment_count(&self) -> D2DResult<u32> {
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
    pub fn figure_count(&self) -> D2DResult<u32> {
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
pub struct PathBuilder {
    path: Path,
    sink: ComPtr<ID2D1GeometrySink>,
}

impl PathBuilder {
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
    pub fn begin_figure(
        self,
        start: impl Into<Point2f>,
        begin: FigureBegin,
        end: FigureEnd,
    ) -> FigureBuilder {
        unsafe {
            self.sink.BeginFigure(start.into().into(), begin as u32);
        }
        FigureBuilder {
            builder: self,
            end: (end as u32),
        }
    }

    #[inline]
    pub fn with_figure(
        self,
        start: impl Into<Point2f>,
        begin: FigureBegin,
        end: FigureEnd,
        f: impl FnOnce(FigureBuilder) -> FigureBuilder,
    ) -> Self {
        f(self.begin_figure(start, begin, end)).end()
    }

    #[inline]
    pub fn with_line_figure(self, begin: FigureBegin, end: FigureEnd, lines: &[Point2f]) -> Self {
        assert!(lines.len() > 1);
        self.with_figure(lines[0], begin, end, |figure| figure.add_lines(&lines[1..]))
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
    pub fn finish(mut self) -> D2DResult<Path> {
        unsafe {
            let hr = self.sink.Close();

            // Move path out of self without invoking drop.
            let path = std::ptr::read(&self.path);
            std::ptr::drop_in_place(&mut self.sink);
            std::mem::forget(self);

            if SUCCEEDED(hr) {
                Ok(path)
            } else {
                Err(hr.into())
            }
        }
    }
}

impl Drop for PathBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.sink.Close();
        }
    }
}

pub struct FigureBuilder {
    builder: PathBuilder,
    end: D2D1_FIGURE_END,
}

impl FigureBuilder {
    #[inline]
    pub fn end(self) -> PathBuilder {
        unsafe {
            self.builder.sink.EndFigure(self.end);

            // Move builder out of self without invoking drop.
            let builder = std::ptr::read(&self.builder);
            std::mem::forget(self);
            builder
        }
    }

    #[inline]
    pub fn add_line<P: Into<Point2f>>(self, point: P) -> Self {
        unsafe { self.builder.sink.AddLine(point.into().into()) };
        self
    }

    #[inline]
    pub fn add_lines(self, points: &[Point2f]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddLines(points.as_ptr() as *const _, points.len() as u32)
        };
        self
    }

    #[inline]
    pub fn add_bezier(self, bezier: &BezierSegment) -> Self {
        unsafe { self.builder.sink.AddBezier(bezier as *const _ as *const _) };
        self
    }

    #[inline]
    pub fn add_beziers(self, beziers: &[BezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    #[inline]
    pub fn add_quadratic_bezier(self, bezier: &QuadBezierSegment) -> Self {
        unsafe {
            self.builder
                .sink
                .AddQuadraticBezier(bezier as *const _ as *const _)
        };
        self
    }

    #[inline]
    pub fn add_quadratic_beziers(self, beziers: &[QuadBezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddQuadraticBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    #[inline]
    pub fn add_arc(self, arc: &ArcSegment) -> Self {
        unsafe { self.builder.sink.AddArc(arc as *const _ as *const _) };
        self
    }
}

impl Drop for FigureBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.builder.sink.EndFigure(self.end);
        }
    }
}
