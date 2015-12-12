use std::sync::Arc;
use winapi::*;
use comptr::ComPtr;
use load_dll;
use error::D2D1Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Factory {
    ptr: ComPtr<ID2D1Factory>,
    lib: Arc<load_dll::D2D1>,
}

impl Factory {
    pub fn create() -> Result<Factory, D2D1Error> {
        let d2d1 = match load_dll::D2D1::load() {
            Ok(d2d1) => Arc::new(d2d1),
            Err(_) => return Err(D2D1Error::MissingLibrary),
        };
        
        let mut ptr: ComPtr<ID2D1Factory> = ComPtr::new(); 
        unsafe {
            let hr = d2d1.create_factory(
                D2D1_FACTORY_TYPE_MULTI_THREADED,
                &ptr.iid(),
                &D2D1_FACTORY_OPTIONS {
                    debugLevel: D2D1_DEBUG_LEVEL_WARNING
                },
                ptr.raw_void()
            );
            
            if !SUCCEEDED(hr) {
                return Err(From::from(hr));
            }
        }
        
        Ok(Factory {
            ptr: ptr,
            lib: d2d1,
        })
    }
}
