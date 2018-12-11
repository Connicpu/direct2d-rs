use crate::brush::gradient::{GradientStop, GradientStopBuilder, GradientStopCollection};
use crate::enums::*;
use crate::error::D2DResult;
use crate::properties::{BrushProperties, RadialGradientBrushProperties};
use crate::render_target::RenderTarget;
use math2d::{Matrix3x2f, Point2f};

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
    pub fn create<'a>(context: &'a RenderTarget) -> RadialGradientBrushBuilder<'a> {
        RadialGradientBrushBuilder::new(context)
    }

    #[inline]
    pub fn get_center(&self) -> Point2f {
        unsafe { self.ptr.GetCenter() }.into()
    }

    #[inline]
    pub fn get_gradient_origin_offset(&self) -> Point2f {
        unsafe { self.ptr.GetGradientOriginOffset() }.into()
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
    pub fn set_center(&self, center: Point2f) {
        unsafe { self.ptr.SetCenter(center.into()) }
    }

    #[inline]
    pub fn set_gradient_origin_offset(&self, offset: Point2f) {
        unsafe { self.ptr.SetGradientOriginOffset(offset.into()) }
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

pub struct RadialGradientBrushBuilder<'a> {
    context: &'a RenderTarget,
    properties: BrushProperties,
    radial_properties: RadialGradientBrushProperties,
    stops: Either<GradientStopBuilder<'a>, &'a GradientStopCollection>,
}

impl<'a> RadialGradientBrushBuilder<'a> {
    #[inline]
    pub fn new(context: &'a RenderTarget) -> Self {
        RadialGradientBrushBuilder {
            context,
            properties: Default::default(),
            radial_properties: Default::default(),
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
                (&self.radial_properties) as *const _ as *const _,
                (&self.properties) as *const _ as *const _,
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
        self.properties.opacity = opacity;
        self
    }

    #[inline]
    pub fn with_transform(mut self, transform: Matrix3x2f) -> Self {
        self.properties.transform = transform;
        self
    }

    #[inline]
    pub fn with_center(mut self, center: Point2f) -> Self {
        self.radial_properties.center = center;
        self
    }

    #[inline]
    pub fn with_origin_offset(mut self, origin_offset: Point2f) -> Self {
        self.radial_properties.origin_offset = origin_offset;
        self
    }

    #[inline]
    pub fn with_radius(mut self, radius_x: f32, radius_y: f32) -> Self {
        self.radial_properties.radius_x = radius_x;
        self.radial_properties.radius_y = radius_y;
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
