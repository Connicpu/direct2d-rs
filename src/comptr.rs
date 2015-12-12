use winapi::*;
use std::{ptr, mem};
use std::cmp::PartialEq;
use std::ops::{Drop, Deref, DerefMut};

pub trait ComUnknown {
    unsafe fn add_ref(ptr: *mut Self) -> ULONG;
    unsafe fn release(ptr: *mut Self) -> ULONG;
    unsafe fn query_interface(ptr: *mut Self, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;
}

pub trait HasIID {
    fn iid() -> IID;
}

// Base types
impl_com_refcount! { IUnknown, "00000000-0000-0000-C000-000000000046" }
impl_com_refcount! { ID2D1Factory, "06152247-6f50-465a-9245-118bfd3b6007" }
impl_com_refcount! { ID2D1RenderTarget, "2cd90694-12e2-11dc-9fed-001143a055f9" }

// Brushes
impl_com_refcount! { ID2D1Brush, "2cd906a8-12e2-11dc-9fed-001143a055f9" }
impl_com_refcount! { ID2D1SolidColorBrush, "2cd906a9-12e2-11dc-9fed-001143a055f9" }

// Geometry
impl_com_refcount! { ID2D1Geometry, "2cd906a1-12e2-11dc-9fed-001143a055f9" }
impl_com_refcount! { ID2D1RectangleGeometry, "2cd906a2-12e2-11dc-9fed-001143a055f9" }
impl_com_refcount! { ID2D1RoundedRectangleGeometry, "2cd906a3-12e2-11dc-9fed-001143a055f9" }
impl_com_refcount! { ID2D1EllipseGeometry, "2cd906a4-12e2-11dc-9fed-001143a055f9" }
impl_com_refcount! { ID2D1GeometryGroup, "2cd906a6-12e2-11dc-9fed-001143a055f9" }
impl_com_refcount! { ID2D1TransformedGeometry, "2cd906bb-12e2-11dc-9fed-001143a055f9" }

// Stroke
impl_com_refcount! { ID2D1StrokeStyle, "2cd9069d-12e2-11dc-9fed-001143a055f9" }

#[derive(Debug)]
pub struct ComPtr<T: ComUnknown> {
    ptr: *mut T,
}

impl<T: ComUnknown> PartialEq for ComPtr<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.ptr == rhs.ptr
    }
}

impl<T: ComUnknown + HasIID> ComPtr<T> {
    pub fn iid(&self) -> IID {
        T::iid()
    }
}

#[allow(dead_code)] // I'm not done, I'll need at least some of it :P
impl<T: ComUnknown> ComPtr<T> {
    pub fn new() -> Self {
        ComPtr { ptr: ptr::null_mut() }
    }
    
    pub fn release(&mut self) {
        unsafe {
            if self.ptr != ptr::null_mut() {
                ComUnknown::release(self.ptr);
                self.ptr = ptr::null_mut();
            }
        }
    }
    
    pub fn is_null(&self) -> bool {
        self.ptr == ptr::null_mut()
    }
    
    pub fn query_interface<U: ComUnknown + HasIID>(&self) -> Result<ComPtr<U>, HRESULT> {
        unsafe {
            if self.ptr == ptr::null_mut() {
                return Err(From::from(E_POINTER));
            }
            
            let mut ptr: ComPtr<U> = ComPtr::new();
            let iid = U::iid();
            let hr = ComUnknown::query_interface(self.ptr, &iid, ptr.raw_void());
            if SUCCEEDED(hr) {
                Ok(ptr)
            } else {
                return Err(From::from(hr));
            }
        }
    }
    
    pub unsafe fn from_existing(ptr: *mut T) -> Self {
        let temp = ComPtr { ptr: ptr };
        mem::forget(temp.clone());
        temp
    }
    
    pub unsafe fn attach(ptr: *mut T) -> Self {
        ComPtr { ptr: ptr }
    }
    
    pub unsafe fn detach(&mut self) -> *mut T {
        let ptr = self.ptr;
        self.ptr = ptr::null_mut();
        ptr
    }
    
    pub unsafe fn raw_value(&self) -> *mut T {
        self.ptr
    }
    
    pub unsafe fn raw_addr(&mut self) -> *mut *mut T {
        assert!(self.ptr == ptr::null_mut());
        &mut self.ptr
    }
    
    pub unsafe fn raw_void(&mut self) -> *mut *mut c_void {
        assert!(self.ptr == ptr::null_mut());
        self.raw_addr() as *mut *mut c_void
    }
}

impl<T: ComUnknown> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        unsafe {
            if self.ptr != ptr::null_mut() {
                ComUnknown::add_ref(self.ptr);
            }
            ComPtr { ptr: self.ptr }
        }
    }
}

impl<T: ComUnknown> Deref for ComPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        assert!(self.ptr != ptr::null_mut());
        unsafe { &*self.ptr }
    }
}

impl<T: ComUnknown> DerefMut for ComPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        assert!(self.ptr != ptr::null_mut());
        unsafe { &mut *self.ptr }
    }
}

impl<T: ComUnknown> Drop for ComPtr<T> {
    fn drop(&mut self) {
        self.release();
    }
}
