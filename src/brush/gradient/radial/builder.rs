use crate::descriptions::GradientStop;
use crate::brush::gradient::radial::RadialGradientBrush;
use crate::brush::gradient::stops::{GradientStopBuilder, GradientStopCollection};
use crate::enums::*;
use crate::error::D2DResult;
use crate::properties::{BrushProperties, RadialGradientBrushProperties};
use crate::render_target::RenderTarget;
use math2d::{Matrix3x2f, Point2f};

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;

pub struct RadialGradientBrushBuilder<'a> {
    context: &'a RenderTarget,
    properties: BrushProperties,
    radial_properties: RadialGradientBrushProperties,
    stops: Stops<'a>,
}

enum Stops<'a> {
    Builder(GradientStopBuilder<'a>),
    Collection(&'a GradientStopCollection),
}

impl<'a> RadialGradientBrushBuilder<'a> {
    #[inline]
    pub fn new(context: &'a RenderTarget) -> Self {
        RadialGradientBrushBuilder {
            context,
            properties: Default::default(),
            radial_properties: Default::default(),
            stops: Stops::Builder(GradientStopBuilder::new(context)),
        }
    }

    #[inline]
    pub fn build(self) -> D2DResult<RadialGradientBrush> {
        let stops = match self.stops {
            Stops::Builder(builder) => builder.build()?,
            Stops::Collection(col) => col.clone(),
        };

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
    /// Sets how colors are determined for pixels below position 0.0 and above 1.0
    /// in the gradient.
    /// It's not valid to call this method if you are using `with_stop_collection`.
    pub fn with_extend_mode(mut self, mode: ExtendMode) -> Self {
        self.stops = match self.stops {
            Stops::Builder(builder) => Stops::Builder(builder.with_extend_mode(mode)),
            Stops::Collection(_) => panic!("Can't modify stops if collection was specified"),
        };
        self
    }

    #[inline]
    /// Sets the gamma mode for the colors when creating a new gradient stop collection.
    /// It's not valid to call this method if you are using `with_stop_collection`.
    pub fn with_gamma(mut self, gamma: Gamma) -> Self {
        self.stops = match self.stops {
            Stops::Builder(builder) => Stops::Builder(builder.with_gamma(gamma)),
            Stops::Collection(_) => panic!("Can't modify stops if collection was specified"),
        };
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
        self.stops = match self.stops {
            Stops::Builder(builder) => Stops::Builder(builder.with_stop(stop)),
            Stops::Collection(_) => panic!("Can't modify stops if collection was specified"),
        };
        self
    }

    #[inline]
    /// Appends a slice onto the list of stops which will be used to create the collection.
    /// It is most efficient to call this method exactly once without using `with_stop` when
    /// that is possible.
    /// It's not valid to call this method if you are using `with_stop_collection`.
    pub fn with_stops(mut self, stops: &'a [GradientStop]) -> Self {
        self.stops = match self.stops {
            Stops::Builder(builder) => Stops::Builder(builder.with_stops(stops)),
            Stops::Collection(_) => panic!("Can't modify stops if collection was specified"),
        };
        self
    }

    #[inline]
    /// Specifies that a pre-existing gradient stop collection should be used
    pub fn with_stop_collection(mut self, stops: &'a GradientStopCollection) -> Self {
        self.stops = Stops::Collection(stops);
        self
    }
}
