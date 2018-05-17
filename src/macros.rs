macro_rules! com_wrapper {
    (@base $ty:ident : $ptrty:ty) => {
        impl $ty {
            /// Initializes the object from an existing wrapped pointer
            #[inline]
            pub unsafe fn from_ptr(ptr: ComPtr<$ptrty>) -> Self {
                $ty { ptr }
            }
            /// Wraps this type from a raw pointer. Takes ownership for refcount purposes.
            #[inline]
            pub unsafe fn from_raw(raw: *mut $ptrty) -> Self {
                Self {
                    ptr: ::wio::com::ComPtr::from_raw(raw),
                }
            }
            /// Gets the underlying raw pointer for this object
            #[inline]
            pub unsafe fn get_raw(&self) -> *mut $ptrty {
                self.ptr.as_raw()
            }
        }
        impl ::std::fmt::Debug for $ty {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.debug_tuple(stringify!($ty))
                    .field(&self.ptr.as_raw())
                    .finish()
            }
        }
    };
    
    ($wrap:ident: $raw:ty) => {
        com_wrapper!($wrap : $raw, send: true, sync: true);
    };
    
    ($wrap:ident: $raw:ty, send: true, sync: true) => {
        com_wrapper!(@base $wrap : $raw);
        unsafe impl Send for $wrap {}
        unsafe impl Sync for $wrap {}
    };

    ($wrap:ident: $raw:ty, send: true, sync: false) => {
        com_wrapper!(@base $wrap : $raw);
        unsafe impl Send for $wrap {}
    };

    ($wrap:ident: $raw:ty, send: false, sync: false) => {
        com_wrapper!(@base $wrap : $raw);
    };
}

macro_rules! brush_type {
    ($ty:ident : $ptrty:ty) => {
        com_wrapper!($ty: $ptrty);

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
    };
}

macro_rules! geometry_type {
    ($ty:ident : $ptrty:ty) => {
        com_wrapper!($ty: $ptrty);

        impl $crate::geometry::Geometry for $ty {
            #[inline]
            unsafe fn get_ptr(&self) -> *mut ::winapi::um::d2d1::ID2D1Geometry {
                self.ptr.as_raw() as *mut _
            }
        }
    };
}

macro_rules! math_wrapper {
    ($(#[$attr:meta])* pub struct $ty:ident(pub $innerty:ty);) => {
        #[derive(Copy, Clone)]
        #[repr(C)]
        $(#[$attr])*
        pub struct $ty(pub $innerty);
        impl ::std::ops::Deref for $ty {
            type Target = $innerty;
            #[inline]
            fn deref(&self) -> &$innerty {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for $ty {
            #[inline]
            fn deref_mut(&mut self) -> &mut $innerty {
                &mut self.0
            }
        }
    };

    ($ty:ident : $innerty:ty) => {
        impl ::std::ops::Deref for $ty {
            type Target = $innerty;
            #[inline]
            fn deref(&self) -> &$innerty {
                &self.0
            }
        }
        
        impl ::std::ops::DerefMut for $ty {
            #[inline]
            fn deref_mut(&mut self) -> &mut $innerty {
                &mut self.0
            }
        }
    };
}

macro_rules! math_wrappers {
    ($($(#[$attr:meta])* pub struct $ty:ident(pub $innerty:ty));+;) => {
        $(
            math_wrapper! { $(#[$attr])* pub struct $ty ( pub $innerty ); }
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

            #[inline]
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

macro_rules! d2d_flags {
    ($(
        #[repr($inner:ident)]
        $(#[$attr:meta])*
        pub enum $flagty:ident {
            $($(#[$vattr:meta])* $name:ident = $value:expr,)*
        }
    )*) => {$(
        #[repr(C)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        $(#[$attr])*
        pub struct $flagty(pub $inner);
        impl $flagty {
            pub const NONE : $flagty = $flagty ( 0 );
            $($(#[$vattr])* pub const $name : $flagty = $flagty ( $value );)*
            #[inline(always)]
            pub fn is_set(self, flag: Self) -> bool {
                self & flag == flag
            }
            #[inline(always)]
            pub fn clear(&mut self, flag: Self) {
                *self &= !flag;
            }
            #[inline(always)]
            pub fn validate(self) -> bool {
                const MASK: $inner = $($value)|*;
                self.0 & !MASK == 0
            }
        }
        impl $crate::std::ops::Not for $flagty {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self {
                $flagty ( !self.0 )
            }
        }
        impl $crate::std::ops::BitAnd for $flagty {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                $flagty ( self.0 & rhs.0 )
            }
        }
        impl $crate::std::ops::BitAndAssign for $flagty {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
        impl $crate::std::ops::BitOr for $flagty {
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                $flagty ( self.0 | rhs.0 )
            }
        }
        impl $crate::std::ops::BitOrAssign for $flagty {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl $crate::std::ops::BitXor for $flagty {
            type Output = Self;
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                $flagty ( self.0 ^ rhs.0 )
            }
        }
        impl $crate::std::ops::BitXorAssign for $flagty {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
        impl $crate::std::fmt::Debug for $flagty {
            fn fmt(&self, fmt: &mut $crate::std::fmt::Formatter) -> $crate::std::fmt::Result {
                fmt.write_str(concat!(stringify!($flagty), "("))?;
                let mut first = true;
                $(if self.is_set($flagty :: $name) {
                    if first {
                        first = false;
                    } else {
                        fmt.write_str(" | ")?;
                    }
                    fmt.write_str(stringify!($name))?;
                })*
                if first {
                    fmt.write_str("NONE")?;
                }
                fmt.write_str(")")?;
                Ok(())
            }
        }
    )*}
}
