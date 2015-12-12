use std::{ptr, slice, mem};
use winapi::*;
use kernel32;

pub trait GetRaw {
    type Raw;
    unsafe fn get_raw(&self) -> *mut Self::Raw;
}

pub trait FromRaw {
    type Raw;
    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;
}

pub fn hresult_from_win32(win32: DWORD) -> HRESULT {
    if win32 as HRESULT <= 0 {
        win32 as HRESULT
    } else {
        ((win32 & 0x0000FFFF) | ((FACILITY_WIN32 as DWORD) << 16) | 0x80000000) as HRESULT
    }
}

pub fn uuid_to_iid(uuid: ::uuid::Uuid) -> IID {
    let mut iid: IID = unsafe { mem::transmute(uuid) };
    iid.Data1 = u32::from_be(iid.Data1);
    iid.Data2 = u16::from_be(iid.Data2);
    iid.Data3 = u16::from_be(iid.Data3);
    
    iid
}

pub fn hresult_to_string(hr: HRESULT) -> Option<String> {
    unsafe {
        let mut buffer: *mut u8 = ptr::null_mut();
        let num_chars = kernel32::FormatMessageA(
            FORMAT_MESSAGE_ALLOCATE_BUFFER |
            FORMAT_MESSAGE_FROM_SYSTEM |
            FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            hr as DWORD,
            0, // unknown lang-id, use default
            (&mut buffer) as *mut *mut u8 as *mut i8,
            0, // minimum buffer size
            ptr::null_mut(),
        );
        if num_chars == 0 {
            return None;
        }
        
        let bytes = slice::from_raw_parts(buffer, num_chars as usize);
        let message = String::from_utf8_lossy(bytes).into_owned();
        kernel32::LocalFree(buffer as *mut _);
        
        Some(message)
    }
}

pub fn last_error_hr() -> HRESULT {
    hresult_from_win32(unsafe { kernel32::GetLastError() })
}

pub fn last_error_string() -> Option<String> {
    hresult_to_string(last_error_hr())
}
