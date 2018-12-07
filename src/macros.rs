macro_rules! brush_type {
    ($ty:ident : $ptrty:ty) => {
        impl $ty {
            #[inline]
            pub unsafe fn from_ptr(ptr: ComPtr<$ptrty>) -> Self {
                $ty { ptr }
            }

            #[inline]
            pub unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                Self {
                    ptr: ::wio::com::ComPtr::from_raw(raw),
                }
            }

            #[inline]
            pub unsafe fn get_raw(&self) -> *mut $ptrty {
                self.ptr.as_raw()
            }
        }

        impl ::brush::Brush for $ty {
            #[inline]
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Brush {
                self.ptr.as_raw() as *mut _
            }
        }

        unsafe impl ::directwrite::drawing_effect::DrawingEffect for $ty {
            #[inline]
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
            #[inline]
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Geometry {
                self.ptr.as_raw() as *mut _
            }
        }

        impl $ty {
            #[inline]
            pub unsafe fn from_ptr(ptr: ComPtr<$ptrty>) -> Self {
                $ty { ptr }
            }
            
            #[inline]
            pub unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                Self {
                    ptr: ::wio::com::ComPtr::from_raw(raw),
                }
            }

            #[inline]
            pub unsafe fn get_raw(&self) -> *mut $ptrty {
                self.ptr.as_raw()
            }
        }

        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}
    };
}
