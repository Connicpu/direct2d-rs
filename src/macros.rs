macro_rules! brush_type {
    (pub struct $ty:ident(ID2D1Brush);) => {
        pub struct $ty {
            ptr: $crate::wio::com::ComPtr<::winapi::um::d2d1::ID2D1Brush>,
        }
        
        impl ::brush::Brush for $ty {
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Brush {
                &mut *self.ptr.as_raw()
            }
        }
        
        impl ::helpers::FromRaw for $ty {
            type Raw = ID2D1Brush;
            unsafe fn from_raw(raw: *mut ::winapi::um::d2d1::ID2D1Brush) -> Self {
                $ty {
                    ptr: $crate::wio::com::ComPtr::from_raw(raw),
                }
            }
        }

        unsafe impl ::directwrite::drawing_effect::DrawingEffect for $ty {
            unsafe fn get_effect_ptr(&self) -> *mut ::winapi::um::unknwnbase::IUnknown {
                self.ptr.as_raw() as *mut ::winapi::um::unknwnbase::IUnknown
            }
        }
    };
    (pub struct $ty:ident($ptrty:ty);) => {
        pub struct $ty {
            ptr: $crate::wio::com::ComPtr<$ptrty>,
        }
        
        impl ::brush::Brush for $ty {
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Brush {
                self.ptr.as_raw() as *mut _
            }
        }
        
        impl ::helpers::FromRaw for $ty {
            type Raw = $ptrty;
            unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                $ty {
                    ptr: $crate::wio::com::ComPtr::from_raw(raw),
                }
            }
        }

        unsafe impl ::directwrite::drawing_effect::DrawingEffect for $ty {
            unsafe fn get_effect_ptr(&self) -> *mut ::winapi::um::unknwnbase::IUnknown {
                self.ptr.as_raw() as *mut ::winapi::um::unknwnbase::IUnknown
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
