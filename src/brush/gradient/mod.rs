use enums::*;
use error::D2DResult;
use math::ColorF;
use render_target::RenderTarget;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1GradientStopCollection;
use wio::com::ComPtr;

pub use self::linear::LinearGradientBrush;
pub use self::radial::RadialGradientBrush;

pub mod linear;
pub mod radial;

#[derive(Clone)]
/// Represents an collection of GradientStop objects for linear and radial gradient brushes.
pub struct GradientStopCollection {
    ptr: ComPtr<ID2D1GradientStopCollection>,
}

impl GradientStopCollection {
    #[inline]
    /// Get the number of stops in the collection
    pub fn len(&self) -> u32 {
        unsafe { self.ptr.GetGradientStopCount() }
    }

    #[inline]
    /// Get all of the stop points
    pub fn get_stops(&self) -> Vec<GradientStop> {
        unsafe {
            let len = self.len();
            let mut stops: Vec<GradientStop> = Vec::with_capacity(len as usize);
            self.ptr.GetGradientStops(stops.as_mut_ptr() as *mut _, len);
            stops
        }
    }
}

com_wrapper!(GradientStopCollection: ID2D1GradientStopCollection);

/// The color for a specific point in the gradient
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GradientStop {
    /// The position this color appears along the gradient in [0.0, 1.0]
    pub position: f32,
    /// The color of the gradient at this position
    pub color: ColorF,
}

impl<C> From<(f32, C)> for GradientStop
where
    C: Into<ColorF>,
{
    #[inline]
    fn from((position, color): (f32, C)) -> Self {
        GradientStop {
            position,
            color: color.into(),
        }
    }
}

/// Builder for creating a gradient stop collection
pub struct GradientStopBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    context: &'a R,
    extend_mode: ExtendMode,
    gamma: Gamma,
    state: GradientStopState<'a>,
}

impl<'a, R> GradientStopBuilder<'a, R>
where
    R: RenderTarget + 'a,
{
    #[inline]
    /// Defaults: Gamma::_2_2, ExtendMode::Clamp
    pub fn new(context: &'a R) -> Self {
        GradientStopBuilder {
            context,
            extend_mode: ExtendMode::Clamp,
            gamma: Gamma::_2_2,
            state: GradientStopState::Empty,
        }
    }

    /// Builds the collection
    fn build(self) -> D2DResult<GradientStopCollection> {
        let slice: &[GradientStop] = match self.state {
            GradientStopState::Empty => &[],
            GradientStopState::Stops(ref vec) => vec,
            GradientStopState::StopSlice(slice) => slice,
        };

        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.context.rt().CreateGradientStopCollection(
                slice.as_ptr() as *const _,
                slice.len() as u32,
                self.gamma as u32,
                self.extend_mode as u32,
                &mut ptr,
            );
            if SUCCEEDED(hr) {
                Ok(GradientStopCollection::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets how colors are determined for positions that would be outside [0.0, 1.0] in the
    /// gradient scale. Defaults to [Clamp][1]
    ///
    /// [1]: ../../enums/enum.ExtendMode.html#variant.Clamp
    pub fn with_extend_mode(mut self, mode: ExtendMode) -> Self {
        self.extend_mode = mode;
        self
    }

    /// Sets the gamma mode for the colors in this collection.
    pub fn with_gamma(mut self, gamma: Gamma) -> Self {
        self.gamma = gamma;
        self
    }

    /// Adds a gradient stop
    pub fn with_stop<G>(mut self, stop: G) -> Self
    where
        G: Into<GradientStop>,
    {
        let stop = stop.into();
        self.state = match self.state {
            GradientStopState::Empty => GradientStopState::Stops(vec![stop]),
            GradientStopState::Stops(mut vec) => {
                vec.push(stop);
                GradientStopState::Stops(vec)
            }
            GradientStopState::StopSlice(slice) => {
                GradientStopState::Stops(slice.iter().cloned().chain(Some(stop)).collect())
            }
        };
        self
    }

    /// Adds a slice of gradient stops. Creating the collection doesn't require any extra 
    /// allocations if you call this function exactly once and don't call `with_stop` at all
    pub fn with_stops(mut self, stops: &'a [GradientStop]) -> Self {
        self.state = match self.state {
            GradientStopState::Empty => GradientStopState::StopSlice(stops),
            GradientStopState::Stops(mut vec) => {
                vec.extend(stops);
                GradientStopState::Stops(vec)
            }
            GradientStopState::StopSlice(slice) => {
                GradientStopState::Stops(slice.iter().chain(stops).cloned().collect())
            }
        };
        self
    }
}

enum GradientStopState<'a> {
    Empty,
    Stops(Vec<GradientStop>),
    StopSlice(&'a [GradientStop]),
}
