macro_rules! brush_type {
    ($(#[ $attrs:meta ])* pub struct $ty:ident($ptrty:ty);) => {
        $(#[ $attrs ])*
        #[derive(Clone)]
        pub struct $ty {
            ptr: $crate::wio::com::ComPtr<$ptrty>,
        }

        impl $crate::brush::Brush for $ty {
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Brush {
                self.ptr.as_raw() as *mut _
            }
        }

        impl $crate::helpers::FromRaw for $ty {
            type Raw = $ptrty;
            unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                $ty {
                    ptr: $crate::wio::com::ComPtr::from_raw(raw),
                }
            }
        }

        unsafe impl $crate::directwrite::drawing_effect::DrawingEffect for $ty {
            unsafe fn get_effect_ptr(&self) -> *mut ::winapi::um::unknwnbase::IUnknown {
                self.ptr.as_raw() as *mut ::winapi::um::unknwnbase::IUnknown
            }
        }

        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}
    };
}

macro_rules! geometry_type {
    ($(#[ $attrs:meta ])* pub struct $ty:ident($ptrty:ty);) => {
        $(#[ $attrs ])*
        #[derive(Clone)]
        pub struct $ty {
            ptr: $crate::wio::com::ComPtr<$ptrty>,
        }

        impl $crate::geometry::Geometry for $ty {
            unsafe fn get_ptr(&self) -> *mut ID2D1Geometry {
                self.ptr.as_raw() as *mut _
            }
        }

        impl $crate::helpers::FromRaw for $ty {
            type Raw = $ptrty;
            unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                $ty {
                    ptr: $crate::wio::com::ComPtr::from_raw(raw),
                }
            }
        }

        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}
    }
}

macro_rules! brush_types {
    ($($(#[ $attrs:meta ])* pub struct $ty:ident($ptrty:ty);)*) => {
        $(
            brush_type! { $(#[ $attrs ])* pub struct $ty($ptrty); }
        )*
    }
}

macro_rules! geometry_types {
    ($($(#[ $attrs:meta ])* pub struct $ty:ident($ptrty:ty);)*) => {
        $(
            geometry_type! { $(#[ $attrs ])* pub struct $ty($ptrty); }
        )*
    }
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
