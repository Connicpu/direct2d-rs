use winapi::*;
use std::{ptr, mem};
use std::ops::{Drop, Deref, DerefMut};

pub trait ComRefcounted {
    unsafe fn add_ref(ptr: *mut Self);
    unsafe fn release(ptr: *mut Self);
}

pub struct ComPtr<T: ComRefcounted> {
    ptr: *mut T,
}

impl<T: ComRefcounted> ComPtr<T> {
    pub fn new() -> Self {
        ComPtr { ptr: ptr::null_mut() }
    }
    
    pub fn release(&mut self) {
        unsafe {
            if self.ptr != ptr::null_mut() {
                ComRefcounted::release(self.ptr);
                self.ptr = ptr::null_mut();
            }
        }
    }
    
    pub fn is_null(&self) -> bool {
        self.ptr == ptr::null_mut()
    }
    
    pub unsafe fn from_existing(ptr: *mut T) -> Self {
        let temp = ComPtr { ptr: ptr };
        mem::forget(temp.clone());
        temp
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

impl<T: ComRefcounted> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        unsafe {
            if self.ptr != ptr::null_mut() {
                ComRefcounted::add_ref(self.ptr);
            }
            ComPtr { ptr: self.ptr }
        }
    }
}

impl<T: ComRefcounted> Deref for ComPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        assert!(self.ptr != ptr::null_mut());
        unsafe { &*self.ptr }
    }
}

impl<T: ComRefcounted> DerefMut for ComPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        assert!(self.ptr != ptr::null_mut());
        unsafe { &mut *self.ptr }
    }
}

impl<T: ComRefcounted> Drop for ComPtr<T> {
    fn drop(&mut self) {
        self.release();
    }
}

macro_rules! impl_com_refcount {
    ($ty:ident) => {
        impl ComRefcounted for $ty {
            unsafe fn add_ref(ptr: *mut Self) {
                (*ptr).AddRef();
            }
            
            unsafe fn release(ptr: *mut Self) {
                (*ptr).Release();
            }
        }
    }
}

impl_com_refcount! { IUnknown }
impl_com_refcount! { ID2D1Factory }
impl_com_refcount! { ID2D1RenderTarget }
impl_com_refcount! { ID2D1Brush }
impl_com_refcount! { ID2D1SolidColorBrush }
