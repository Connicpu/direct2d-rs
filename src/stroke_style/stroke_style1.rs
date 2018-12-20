use crate::factory::Factory1;

use winapi::um::d2d1_1::ID2D1StrokeStyle1;
use wio::com::ComPtr;

pub use self::builder::*;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct StrokeStyle1 {
    ptr: ComPtr<ID2D1StrokeStyle1>,
}

impl StrokeStyle1 {
    #[inline]
    pub fn create<'a>(factory: &'a Factory1) -> StrokeStyleBuilder1<'a> {
        StrokeStyleBuilder1::new(factory)
    }
}
