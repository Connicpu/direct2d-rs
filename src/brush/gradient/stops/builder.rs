use crate::brush::gradient::stops::GradientStopCollection;
use crate::descriptions::GradientStop;
use crate::enums::*;
use crate::render_target::IRenderTarget;

use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;

pub struct GradientStopBuilder<'a> {
    context: &'a dyn IRenderTarget,
    extend_mode: ExtendMode,
    gamma: Gamma,
    state: GradientStopState<'a>,
}

impl<'a> GradientStopBuilder<'a> {
    /// Defaults: Gamma::_2_2, ExtendMode::Clamp
    pub fn new(context: &'a dyn IRenderTarget) -> Self {
        GradientStopBuilder {
            context,
            extend_mode: ExtendMode::Clamp,
            gamma: Gamma::_2_2,
            state: GradientStopState::Empty,
        }
    }

    pub fn build(self) -> Result<GradientStopCollection, Error> {
        let slice: &[GradientStop] = match self.state {
            GradientStopState::Empty => &[],
            GradientStopState::Stops(ref vec) => vec,
            GradientStopState::StopSlice(slice) => slice,
        };

        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.context.raw_rt().CreateGradientStopCollection(
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

    pub fn with_extend_mode(mut self, mode: ExtendMode) -> Self {
        self.extend_mode = mode;
        self
    }

    pub fn with_gamma(mut self, gamma: Gamma) -> Self {
        self.gamma = gamma;
        self
    }

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
