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

pub unsafe trait IResource {
    fn factory(&self) -> Factory {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            self.raw_resource().GetFactory(&mut ptr);
            Factory::from_raw(ptr)
        }
    }

    unsafe fn raw_resource(&self) -> &ID2D1Resource;
}

unsafe impl IResource for Resource {
    unsafe fn raw_resource(&self) -> &ID2D1Resource {
        &self.ptr
    }
}
