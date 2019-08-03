use crate::enums::{FigureBegin, FigureEnd, FillMode, PathSegment};
use crate::geometry::path::PathGeometry;

use dcommon::Error;
use math2d::{ArcSegment, BezierSegment, Point2f, QuadBezierSegment};
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1GeometrySink;
use wio::com::ComPtr;

/// Interface for building Path geometry
pub struct PathBuilder {
    pub(super) path: PathGeometry,
    pub(super) sink: ComPtr<ID2D1GeometrySink>,
}

impl PathBuilder {
    pub fn fill_mode(self, fill_mode: FillMode) -> Self {
        unsafe { self.sink.SetFillMode(fill_mode as u32) };
        self
    }

    pub fn set_segment_flags(self, flags: PathSegment) -> Self {
        unsafe { self.sink.SetSegmentFlags(flags as u32) };
        self
    }

    pub fn begin_figure(self, start: impl Into<Point2f>, begin: FigureBegin) -> FigureBuilder {
        unsafe {
            self.sink.BeginFigure(start.into().into(), begin as u32);
        }
        FigureBuilder { builder: self }
    }

    pub fn with_figure(
        self,
        start: impl Into<Point2f>,
        begin: FigureBegin,
        end: FigureEnd,
        f: impl FnOnce(FigureBuilder) -> FigureBuilder,
    ) -> Self {
        f(self.begin_figure(start, begin)).end(end)
    }

    pub fn with_line_figure(self, begin: FigureBegin, end: FigureEnd, lines: &[Point2f]) -> Self {
        assert!(lines.len() > 1);
        self.with_figure(lines[0], begin, end, |figure| figure.add_lines(&lines[1..]))
    }

    pub fn copy_from(self, path: &PathGeometry) -> Result<Self, Error> {
        unsafe {
            let hr = path.ptr.Stream(self.sink.as_raw());
            if SUCCEEDED(hr) {
                Ok(self)
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn finish(mut self) -> Result<PathGeometry, Error> {
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
    fn drop(&mut self) {
        unsafe {
            self.sink.Close();
        }
    }
}

pub struct FigureBuilder {
    builder: PathBuilder,
}

impl FigureBuilder {
    pub fn end(self, end: FigureEnd) -> PathBuilder {
        unsafe {
            self.builder.sink.EndFigure(end as u32);

            // Move builder out of self without invoking drop.
            let builder = std::ptr::read(&self.builder);
            std::mem::forget(self);
            builder
        }
    }

    pub fn add_line<P: Into<Point2f>>(self, point: P) -> Self {
        unsafe { self.builder.sink.AddLine(point.into().into()) };
        self
    }

    pub fn add_lines(self, points: &[Point2f]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddLines(points.as_ptr() as *const _, points.len() as u32)
        };
        self
    }

    pub fn add_bezier(self, bezier: &BezierSegment) -> Self {
        unsafe { self.builder.sink.AddBezier(bezier as *const _ as *const _) };
        self
    }

    pub fn add_beziers(self, beziers: &[BezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    pub fn add_quadratic_bezier(self, bezier: &QuadBezierSegment) -> Self {
        unsafe {
            self.builder
                .sink
                .AddQuadraticBezier(bezier as *const _ as *const _)
        };
        self
    }

    pub fn add_quadratic_beziers(self, beziers: &[QuadBezierSegment]) -> Self {
        unsafe {
            self.builder
                .sink
                .AddQuadraticBeziers(beziers.as_ptr() as *const _, beziers.len() as u32)
        };
        self
    }

    pub fn add_arc(self, arc: &ArcSegment) -> Self {
        unsafe { self.builder.sink.AddArc(arc as *const _ as *const _) };
        self
    }
}

impl Drop for FigureBuilder {
    fn drop(&mut self) {
        unsafe {
            self.builder.sink.EndFigure(FigureEnd::Open as u32);
        }
    }
}
