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
