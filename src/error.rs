use std::fmt;

use dxgi;
use winapi::shared::ntdef::HRESULT;

pub type D2DResult<T> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// A Direct2D API returned an enum value that this abstraction doesn't know about
    UnknownEnumValue,

    /// The error came from a DXGI API
    Dxgi(dxgi::error::Error),

    //TODO: The error came from a DWrite API
    //DWrite(directwrite::error::DWriteError),
    /// Any other HRESULT error
    ComError(HRESULT),
}

impl Error {
    pub fn get_message(&self) -> String {
        match self {
            &Error::UnknownEnumValue => "Unknown enum value".to_string(),
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

impl From<HRESULT> for Error {
    #[inline]
    fn from(hr: HRESULT) -> Error {
        Error::ComError(hr)
    }
}

impl<'a> From<&'a Error> for Error {
    #[inline]
    fn from(e: &'a Error) -> Error {
        e.clone()
    }
}
