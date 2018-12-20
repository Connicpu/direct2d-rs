use crate::factory::Factory;

use com_wrapper::ComWrapper;
use winapi::um::d2d1::ID2D1Resource;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Resource {
    ptr: ComPtr<ID2D1Resource>,
}

impl Resource {
    pub fn factory(&self) -> Factory {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            self.ptr.GetFactory(&mut ptr);
            Factory::from_raw(ptr)
        }
    }
}
