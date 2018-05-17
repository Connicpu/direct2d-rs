use enums::{ArcSize, SweepDirection};
use math::{Point2F, SizeF};

use winapi::um::d2d1::{D2D1_ARC_SEGMENT, D2D1_BEZIER_SEGMENT, D2D1_QUADRATIC_BEZIER_SEGMENT};

#[derive(Copy, Clone)]
#[repr(C)]
/// Represents a cubic bezier segment drawn between two points.
pub struct BezierSegment(pub D2D1_BEZIER_SEGMENT);

math_wrapper!(BezierSegment: D2D1_BEZIER_SEGMENT);

#[derive(Copy, Clone)]
#[repr(C)]
/// Contains the control point and end point for a quadratic Bezier segment.
pub struct QuadBezierSegment(pub D2D1_QUADRATIC_BEZIER_SEGMENT);

math_wrapper!(QuadBezierSegment: D2D1_QUADRATIC_BEZIER_SEGMENT);

#[derive(Copy, Clone)]
#[repr(C)]
/// Describes an elliptical arc between two points.
pub struct ArcSegment(pub D2D1_ARC_SEGMENT);

math_wrapper!(ArcSegment: D2D1_ARC_SEGMENT);

impl BezierSegment {
    #[inline]
    pub fn new(p1: Point2F, p2: Point2F, p3: Point2F) -> BezierSegment {
        BezierSegment(D2D1_BEZIER_SEGMENT {
            point1: p1.0,
            point2: p2.0,
            point3: p3.0,
        })
    }
}

impl QuadBezierSegment {
    #[inline]
    pub fn new(p1: Point2F, p2: Point2F) -> QuadBezierSegment {
        QuadBezierSegment(D2D1_QUADRATIC_BEZIER_SEGMENT {
            point1: p1.0,
            point2: p2.0,
        })
    }
}

impl ArcSegment {
    #[inline]
    pub fn new(
        point: Point2F,
        size: SizeF,
        angle: f32,
        sweep_dir: SweepDirection,
        arc_size: ArcSize,
    ) -> ArcSegment {
        ArcSegment(D2D1_ARC_SEGMENT {
            point: point.0,
            size: size.0,
            rotationAngle: angle,
            sweepDirection: (sweep_dir as u32),
            arcSize: (arc_size as u32),
        })
    }

    /// Create a counter-clockwise small arc
    #[inline]
    pub fn new_cc_sm(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(
            point,
            size,
            angle,
            SweepDirection::CounterClockwise,
            ArcSize::Small,
        )
    }

    /// Create a counter-clockwise large arc
    #[inline]
    pub fn new_cc_lg(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(
            point,
            size,
            angle,
            SweepDirection::CounterClockwise,
            ArcSize::Large,
        )
    }

    /// Create a clockwise small arc
    #[inline]
    pub fn new_cw_sm(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(
            point,
            size,
            angle,
            SweepDirection::Clockwise,
            ArcSize::Small,
        )
    }

    /// Create a counter-clockwise small arc
    #[inline]
    pub fn new_cw_lg(point: Point2F, size: SizeF, angle: f32) -> ArcSegment {
        ArcSegment::new(
            point,
            size,
            angle,
            SweepDirection::Clockwise,
            ArcSize::Large,
        )
    }
}
