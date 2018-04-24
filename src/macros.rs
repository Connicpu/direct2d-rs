macro_rules! brush_type {
    ($ty:ident : $ptrty:ty) => {
        impl $ty {
            pub unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                Self {
                    ptr: ::wio::com::ComPtr::from_raw(raw),
                }
            }

            pub unsafe fn get_raw(&self) -> *mut $ptrty {
                self.ptr.as_raw()
            }
        }

        impl ::brush::Brush for $ty {
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Brush {
                self.ptr.as_raw() as *mut _
            }
        }

        unsafe impl ::directwrite::drawing_effect::DrawingEffect for $ty {
            unsafe fn get_effect_ptr(&self) -> *mut ::winapi::um::unknwnbase::IUnknown {
                self.ptr.as_raw() as *mut ::winapi::um::unknwnbase::IUnknown
            }
        }

        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}
    };
}

macro_rules! geometry_type {
    ($ty:ident : $ptrty:ty) => {
        impl $crate::geometry::Geometry for $ty {
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Geometry {
                self.ptr.as_raw() as *mut _
            }
        }

        impl $ty {
            pub unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                Self {
                    ptr: ::wio::com::ComPtr::from_raw(raw),
                }
            }

            pub unsafe fn get_raw(&self) -> *mut $ptrty {
                self.ptr.as_raw()
            }
        }

        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}
    };
}

macro_rules! math_wrapper {
    (pub struct $ty:ident(pub $innerty:ty);) => {
        #[derive(Copy, Clone)]
        #[repr(C)]
        pub struct $ty(pub $innerty);
        impl ::std::ops::Deref for $ty {
            type Target = $innerty;
            fn deref(&self) -> &$innerty {
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

macro_rules! d2d_enums {
    ($(
        pub enum $ename:ident {
            $($ekey:ident = $eval:expr,)*
        }
    )*) => {$(
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $ename {
            $($ekey = $eval,)*
        }

        impl $ename {
            #[inline(always)]
            pub fn to_u32(self) -> u32 {
                self as u32
            }
            
            pub fn from_u32(value: u32) -> Option<Self> {
                match value {
                    $($eval => Some($ename :: $ekey),)*
                    _ => None,
                }
            }
        }

        impl CheckedEnum for $ename {
            #[inline(always)]
            fn to_u32(self) -> u32 {
                $ename :: to_u32(self)
            }
            #[inline(always)]
            fn from_u32(value: u32) -> Option<Self> {
                $ename :: from_u32(value)
            }
        }
    )*};
}
