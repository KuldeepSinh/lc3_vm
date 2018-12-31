//! `sys` module is designed to handle low level interaction with the system.
//! Most of the times, such interactions require use of unsafe code via libc and other crates.
pub mod errno;
pub mod getchar;
pub mod select;
pub mod terminal;
pub mod time;

use self::errno::Errno;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// Nix Error Type
///
/// The nix error type provides a common way of dealing with
/// various system system/libc calls that might fail.  Each
/// error has a corresponding errno (usually the one from the
/// underlying OS) to which it can be mapped in addition to
/// implementing other common traits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    Sys(Errno),
    InvalidPath,
    /// The operation involved a conversion to Rust's native String type, which failed because the
    /// string did not contain all valid UTF-8.
    InvalidUtf8,
    /// The operation is not supported by Nix, in this instance either use the libc bindings or
    /// consult the module documentation to see if there is a more appropriate interface available.
    UnsupportedOperation,
}

impl Error {
    /// Convert this `Error` to an [`Errno`](enum.Errno.html).
    ///
    pub fn as_errno(&self) -> Option<Errno> {
        if let &Error::Sys(ref e) = self {
            Some(*e)
        } else {
            None
        }
    }

    /// Create a nix Error from a given errno
    pub fn from_errno(errno: Errno) -> Error {
        Error::Sys(errno)
    }

    /// Get the current errno and convert it to a nix Error
    pub fn last() -> Error {
        Error::Sys(Errno::last())
    }

    /// Create a new invalid argument error (`EINVAL`)
    pub fn invalid_argument() -> Error {
        Error::Sys(Errno::EINVAL)
    }
}

impl From<Errno> for Error {
    fn from(errno: Errno) -> Error {
        Error::from_errno(errno)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Error {
        Error::InvalidUtf8
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidPath => "Invalid path",
            Error::InvalidUtf8 => "Invalid UTF-8 string",
            Error::UnsupportedOperation => "Unsupported Operation",
            Error::Sys(ref errno) => errno.desc(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidPath => write!(f, "Invalid path"),
            Error::InvalidUtf8 => write!(f, "Invalid UTF-8 string"),
            Error::UnsupportedOperation => write!(f, "Unsupported Operation"),
            Error::Sys(errno) => write!(f, "{:?}: {}", errno, errno.desc()),
        }
    }
}
