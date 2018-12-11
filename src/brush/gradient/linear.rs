use crate::brush::gradient::{GradientStop, GradientStopBuilder, GradientStopCollection};
use crate::enums::*;
use crate::error::D2DResult;
use math2d::{Matrix3x2f, Point2f};
use crate::render_target::RenderTarget;
use crate::properties::{BrushProperties, LinearGradientBrushProperties};

use std::mem;
use std::ptr;

use either::Either;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1LinearGradientBrush;
use wio::com::ComPtr;

#[derive(Clone)]
/// Paints an area with a linear gradient.
pub struct LinearGradientBrush {
    ptr: ComPtr<ID2D1LinearGradientBrush>,
}

impl LinearGradientBrush {
    #[inline]
    pub fn create<'a>(context: &'a RenderTarget) -> LinearGradientBrushBuilder<'a>
    {
        LinearGradientBrushBuilder::new(context)
    }

    /// Retrieves the starting coordinates of the linear gradient.
    #[inline]
    pub fn get_start_point(&self) -> Point2f {
        unsafe { self.ptr.GetStartPoint().into() }
    }

    /// Retrieves the ending coordinates of the linear gradient.
    #[inline]
    pub fn get_end_point(&self) -> Point2f {
        unsafe { self.ptr.GetEndPoint().into() }
    }

    /// Retrieves the `GradientStopCollection` associated with this linear gradient brush.
    #[inline]
    pub fn get_gradient_stop_collection(&self) -> GradientStopCollection {
        unsafe {
            let mut ptr = ptr::null_mut();
            self.ptr.GetGradientStopCollection(&mut ptr);
            GradientStopCollection {
                ptr: ComPtr::from_raw(ptr),
            }
        }
    }

    /// Sets the starting coordinates of the linear gradient in the brush's coordinate space.
    #[inline]
    pub fn set_start_point(&mut self, point: Point2f) {
        unsafe { self.ptr.SetStartPoint(point.into()) }
    }

    /// Sets the ending coordinates of the linear gradient in the brush's coordinate space.
    #[inline]
    pub fn set_end_point(&mut self, point: Point2f) {
        unsafe { self.ptr.SetEndPoint(point.into()) }
    }
}

brush_type!(LinearGradientBrush: ID2D1LinearGradientBrush);

pub struct LinearGradientBrushBuilder<'a>
{
    context: &'a RenderTarget,
    properties: BrushProperties,
    linear_properties: LinearGradientBrushProperties,
    stops: Either<GradientStopBuilder<'a>, &'a GradientStopCollection>,
}

impl<'a> LinearGradientBrushBuilder<'a>
{
    #[inline]
    /// Creates a builder. It starts with the opacity as 1.0 and the transform as the identity
    /// matrix, and all other values zeroed out. You should add at least two gradient stops.
    pub fn new(context: &'a RenderTarget) -> Self {
        LinearGradientBrushBuilder {
            context,
            properties: BrushProperties::new(1.0, &Matrix3x2f::IDENTITY),
            linear_properties: unsafe { mem::zeroed() },
            stops: Either::Left(GradientStopBuilder::new(context)),
        }
    }

    #[inline]
    /// Build the brush described in this builder. Will likely fail if you haven't added any
    /// stops to the collection (or specified an existing collection).
    pub fn build(self) -> D2DResult<LinearGradientBrush> {
        let stops = self.stops.map_left(|b| b.build());
        let stops = stops.as_ref().either(|b| b.as_ref(), |&c| Ok(c))?;

        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.context.rt().CreateLinearGradientBrush(
                (&self.linear_properties) as *const _ as *const _,
                (&self.properties) as *const _ as *const _,
                stops.get_raw(),
                &mut ptr,
            );
            if SUCCEEDED(hr) {
                Ok(LinearGradientBrush::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    /// Sets the opacity and transform
    pub fn with_properties(mut self, properties: BrushProperties) -> Self {
        self.properties = properties;
        self
    }

    #[inline]
    /// This changes the overall opacity of the brush, which is essentially
    /// multiplied with the colors in the gradient stops
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.properties.opacity = opacity;
        self
    }

    #[inline]
    /// Sets a full affine transform on how the gradient is applied when rendered
    pub fn with_transform(mut self, transform: Matrix3x2f) -> Self {
        self.properties.transform = transform;
        self
    }

    #[inline]
    /// Sets the coordinate where the gradient stop at position 0.0 applies in full
    pub fn with_start(mut self, start: Point2f) -> Self {
        self.linear_properties.start = start;
        self
    }

    #[inline]
    /// Sets the coordinate where the gradient stop at position 1.0 applies in full
    pub fn with_end(mut self, end: Point2f) -> Self {
        self.linear_properties.end = end;
        self
    }

    #[inline]
    /// Sets how colors are determined for pixels below position 0.0 and above 1.0
    /// in the gradient.
    /// It's not valid to call this method if you are using `with_stop_collection`.
    pub fn with_extend_mode(mut self, mode: ExtendMode) -> Self {
        self.stops = Either::Left(
            self.stops
                .left()
                .expect("Can't modify stops if collection was specified")
                .with_extend_mode(mode),
        );
        self
    }

    #[inline]
    /// Sets the gamma mode for the colors when creating a new gradient stop collection.
    /// It's not valid to call this method if you are using `with_stop_collection`.
    pub fn with_gamma(mut self, gamma: Gamma) -> Self {
        self.stops = Either::Left(
            self.stops
                .left()
                .expect("Can't modify stops if collection was specified")
                .with_gamma(gamma),
        );
        self
    }

    #[inline]
    /// Appends an individual GradientStop onto the list of stops which will be used
    /// to create the collection.
    /// It's not valid to call this method if you are using `with_stop_collection`.
    pub fn with_stop<G>(mut self, stop: G) -> Self
    where
        G: Into<GradientStop>,
    {
        self.stops = Either::Left(
            self.stops
                .left()
                .expect("Can't modify stops if collection was specified")
                .with_stop(stop),
        );
        self
    }

    #[inline]
    /// Appends a slice onto the list of stops which will be used to create the collection.
    /// It is most efficient to call this method exactly once without using `with_stop` when
    /// that is possible.
    /// It's not valid to call this method if you are using `with_stop_collection`.
    pub fn with_stops(mut self, stops: &'a [GradientStop]) -> Self {
        self.stops = Either::Left(
            self.stops
                .left()
                .expect("Can't modify stops if collection was specified")
                .with_stops(stops),
        );
        self
    }

    #[inline]
    /// Specifies that a pre-existing gradient stop collection should be used
    pub fn with_stop_collection(mut self, stops: &'a GradientStopCollection) -> Self {
        self.stops = Either::Right(stops);
        self
    }
}
