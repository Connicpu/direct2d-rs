pub trait GetRaw {
    type Raw;
    unsafe fn get_raw(&self) -> *mut Self::Raw;
}

pub trait FromRaw {
    type Raw;
    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;
}
