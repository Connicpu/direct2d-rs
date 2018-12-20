use crate::error::D2DResult;
use crate::render_target::RenderTarget;

use std::ptr;

use com_wrapper::ComWrapper;
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
    #[inline]
    pub fn create(target: &RenderTarget, size: Option<&Sizef>) -> D2DResult<Layer> {
        let size = match size {
            Some(size) => size as *const _ as *const _,
            None => ptr::null(),
        };

        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = target.rt().CreateLayer(size, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(Layer::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    #[inline]
    pub fn size(&self) -> Sizef {
        unsafe { self.ptr.GetSize().into() }
    }
}
