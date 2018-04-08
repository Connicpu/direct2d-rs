use error::Error;

use winapi::shared::winerror::{HRESULT, SUCCEEDED};

pub trait GetRaw {
    type Raw;
    unsafe fn get_raw(&self) -> *mut Self::Raw;
}

pub trait FromRaw {
    type Raw;
    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;
}

pub unsafe fn ret_obj<T, P>(hr: HRESULT, ptr: *mut P) -> Result<T, Error>
where
    T: FromRaw<Raw = P>,
{
    if SUCCEEDED(hr) {
        Ok(FromRaw::from_raw(ptr))
    } else {
        Err(From::from(hr))
    }
}
