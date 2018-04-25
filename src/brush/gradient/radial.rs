use brush::gradient::{GradientStop, GradientStopBuilder, GradientStopCollection};
use enums::*;
use error::D2DResult;
use math::{BrushProperties, Matrix3x2F, Point2F, RadialGradientBrushProperties};
use render_target::RenderTarget;

use std::mem;
use std::ptr;

use either::Either;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1RadialGradientBrush;
use wio::com::ComPtr;

#[derive(Clone)]
/// Paints an area with a linear gradient.
pub struct RadialGradientBrush {
    ptr: ComPtr<ID2D1RadialGradientBrush>,
}

impl RadialGradientBrush {
    #[inline]
    pub fn create<'a, R>(context: &'a R) -> RadialGradientBrushBuilder<'a, R>
    where
        R: RenderTarget + 'a,
    {
        RadialGradientBrushBuilder::new(context)
    }

    #[inline]
    pub fn get_center(&self) -> Point2F {
        Point2F(unsafe { self.ptr.GetCenter() })
    }

    #[inline]
    pub fn get_gradient_origin_offset(&self) -> Point2F {
        Point2F(unsafe { self.ptr.GetGradientOriginOffset() })
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

    #[inline]
    pub fn get_radius_x(&self) -> f32 {
        unsafe { self.ptr.GetRadiusX() }
    }

    #[inline]
    pub fn get_radius_y(&self) -> f32 {
        unsafe { self.ptr.GetRadiusY() }
    }

    #[inline]
    pub fn set_center(&self, center: Point2F) {
        unsafe { self.ptr.SetCenter(center.0) }
    }

    #[inline]
    pub fn set_gradient_origin_offset(&self, offset: Point2F) {
        unsafe { self.ptr.SetGradientOriginOffset(offset.0) }
    }

    #[inline]
    pub fn set_radius_x(&self, radius: f32) {
        unsafe { self.ptr.SetRadiusX(radius) }
    }

    #[inline]
    pub fn set_radius_y(&self, radius: f32) {
        unsafe { self.ptr.SetRadiusY(radius) }
    }
}

brush_type!(RadialGradientBrush: ID2D1RadialGradientBrush);

pub struct RadialGradientBrushBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    context: &'a R,
    properties: BrushProperties,
    radial_properties: RadialGradientBrushProperties,
    stops: Either<GradientStopBuilder<'a, R>, &'a GradientStopCollection>,
}

impl<'a, R> RadialGradientBrushBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    #[inline]
    pub fn new(context: &'a R) -> Self {
        RadialGradientBrushBuilder {
            context,
            properties: BrushProperties::new(1.0, &Matrix3x2F::IDENTITY),
            radial_properties: unsafe { mem::zeroed() },
            stops: Either::Left(GradientStopBuilder::new(context)),
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<RadialGradientBrush> {
        let stops = self.stops.map_left(|b| b.build());
        let stops = stops.as_ref().either(|b| b.as_ref(), |&c| Ok(c))?;

        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.context.rt().CreateRadialGradientBrush(
                &self.radial_properties.0,
                &self.properties.0,
                stops.get_raw(),
                &mut ptr,
            );
            if SUCCEEDED(hr) {
                Ok(RadialGradientBrush::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn with_properties(mut self, properties: BrushProperties) -> Self {
        self.properties = properties;
        self
    }

    #[inline]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.properties.0.opacity = opacity;
        self
    }

    #[inline]
    pub fn with_transform(mut self, transform: Matrix3x2F) -> Self {
        self.properties.0.transform = transform.0;
        self
    }

    #[inline]
    pub fn with_center(mut self, center: Point2F) -> Self {
        self.radial_properties.center = center.0;
        self
    }

    #[inline]
    pub fn with_origin_offset(mut self, origin_offset: Point2F) -> Self {
        self.radial_properties.gradientOriginOffset = origin_offset.0;
        self
    }

    #[inline]
    pub fn with_radius(mut self, radius_x: f32, radius_y: f32) -> Self {
        self.radial_properties.radiusX = radius_x;
        self.radial_properties.radiusY = radius_y;
        self
    }

    #[inline]
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
    pub fn with_stop_collection(mut self, stops: &'a GradientStopCollection) -> Self {
        self.stops = Either::Right(stops);
        self
    }
}
