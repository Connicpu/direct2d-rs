use std::fmt;

use dxgi;
use winapi::shared::ntdef::HRESULT;

pub type D2DResult<T> = Result<T, Error>;

#[derive(Copy, Clone, PartialEq)]
pub enum Error {
    /// The error came from a DXGI API
    Dxgi(dxgi::error::Error),

    /// Any other HRESULT error
    ComError(HRESULT),
}

impl Error {
    pub fn get_message(&self) -> String {
        match self {
            &Error::Dxgi(dxgierr) => dxgierr.get_message(),
            &Error::ComError(hr) => dxgi::error::Error(hr).get_message(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.get_message())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Error")
            .field(&i32::from(*self))
            .field(&self.get_message())
            .finish()
    }
}

impl From<HRESULT> for Error {
    #[inline]
    fn from(hr: HRESULT) -> Error {
        Error::ComError(hr)
    }
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::Dxgi(e) => e.0,
            Error::ComError(e) => e,
        }
    }
}

impl<'a> From<&'a Error> for Error {
    #[inline]
    fn from(e: &'a Error) -> Error {
        e.clone()
    }
}
