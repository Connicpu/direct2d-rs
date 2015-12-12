use winapi::*;

#[derive(Clone, Debug, PartialEq)]
pub enum D2D1Error {
    /// May be caused if you try to run this on an older version of windows
    MissingLibrary,
    /// Any other HRESULT error
    ComError(HRESULT),
}

impl From<HRESULT> for D2D1Error {
    fn from(hr: HRESULT) -> D2D1Error {
        D2D1Error::ComError(hr)
    }
}
