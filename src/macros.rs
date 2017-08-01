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
    (pub struct $ty:ident(ID2D1Brush);) => {
        pub struct $ty {
            ptr: $crate::comptr::ComPtr<::winapi::um::d2d1::ID2D1Brush>,
        }
        
        impl ::brush::Brush for $ty {
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Brush {
                &mut *self.ptr.raw_value()
            }
        }
        
        impl ::helpers::FromRaw for $ty {
            type Raw = ID2D1Brush;
            unsafe fn from_raw(raw: *mut ::winapi::um::d2d1::ID2D1Brush) -> Self {
                $ty {
                    ptr: $crate::comptr::ComPtr::from_existing(raw),
                }
            }
        }

        unsafe impl ::directwrite::drawing_effect::DrawingEffect for $ty {
            unsafe fn get_effect_ptr(&self) -> *mut ::winapi::um::unknwnbase::IUnknown {
                self.ptr.raw_value() as *mut ::winapi::um::unknwnbase::IUnknown
            }
        }
    };
    (pub struct $ty:ident($ptrty:ty);) => {
        pub struct $ty {
            ptr: $crate::comptr::ComPtr<$ptrty>,
        }
        
        impl ::brush::Brush for $ty {
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Brush {
                self.ptr.raw_value() as *mut _
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

        unsafe impl ::directwrite::drawing_effect::DrawingEffect for $ty {
            unsafe fn get_effect_ptr(&self) -> *mut ::winapi::um::unknwnbase::IUnknown {
                self.ptr.raw_value() as *mut ::winapi::um::unknwnbase::IUnknown
            }
        }
    };
}

macro_rules! math_wrapper {
    (pub struct $ty:ident(pub $innerty:ty);) => {
        #[derive(Copy, Clone)] #[repr(C)]
        pub struct $ty(pub $innerty);
        impl ::std::ops::Deref for $ty {
            type Target = $innerty;
            fn deref(&self) -> & $innerty {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut $innerty {
                &mut self.0
            }
        }
    };
}

macro_rules! math_wrappers {
    ($(pub struct $ty:ident(pub $innerty:ty));+;) => {
        $(
            math_wrapper! { pub struct $ty ( pub $innerty ); }
        )+
    }
}
