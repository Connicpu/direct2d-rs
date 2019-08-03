use crate::render_target::IRenderTarget;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Sizef;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1::ID2D1Layer;
use wio::com::ComPtr;

pub use self::builder::LayerBuilder;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct Layer {
    ptr: ComPtr<ID2D1Layer>,
}

impl Layer {
    pub fn create(target: &dyn IRenderTarget, size: Option<&Sizef>) -> Result<Layer, Error> {
        let size = match size {
            Some(size) => size as *const _ as *const _,
            None => std::ptr::null(),
        };

        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = target.raw_rt().CreateLayer(size, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(Layer::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn size(&self) -> Sizef {
        unsafe { self.ptr.GetSize().into() }
    }
}
