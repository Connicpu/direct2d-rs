use crate::geometry::path::PathGeometry;
use crate::enums::{FigureBegin, FigureEnd, FillMode, PathSegment};
use crate::error::D2DResult;

use math2d::{ArcSegment, BezierSegment, Point2f, QuadBezierSegment};
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::{ID2D1GeometrySink,  D2D1_FIGURE_END};
use wio::com::ComPtr;

/// Interface for building Path geometry
pub struct PathBuilder {
    pub(super) path: PathGeometry,
    pub(super) sink: ComPtr<ID2D1GeometrySink>,
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
    pub fn copy_from(self, path: &PathGeometry) -> D2DResult<Self> {
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
    pub fn finish(mut self) -> D2DResult<PathGeometry> {
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
