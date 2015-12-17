// To make COM types usable from ComPtr
macro_rules! impl_com_refcount {
    ($ty:ident) => {
        impl ComUnknown for $ty {
            unsafe fn add_ref(ptr: *mut Self) -> ULONG {
                (*ptr).AddRef()
            }
            
            unsafe fn release(ptr: *mut Self) -> ULONG {
                (*ptr).Release()
            }
            
            unsafe fn query_interface(ptr: *mut Self, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT {
                (*ptr).QueryInterface(riid, ppv)
            }
        }
    };
    ($ty:ident, $uuid:expr) => {
        impl_com_refcount! { $ty }
        impl HasIID for $ty {
            fn iid() -> IID {
                let uuid = ::uuid::Uuid::parse_str($uuid).unwrap();
                $crate::helpers::uuid_to_iid(uuid)
            }
        }
    };
}

macro_rules! brush_type {
    (pub struct $ty:ident($ptrty:ty);) => {
        pub struct $ty {
            ptr: $crate::comptr::ComPtr<$ptrty>,
        }
        
        impl ::brush::Brush for $ty {
            unsafe fn get_ptr(&self) -> *mut ID2D1Brush {
                &mut **self.ptr.raw_value()
            }
        }
        
        impl ::helpers::FromRaw for $ty {
            type Raw = $ptrty;
            unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                $ty {
                    ptr: $crate::comptr::ComPtr::from_existing(raw),
                }
            }
        }
    }
}
