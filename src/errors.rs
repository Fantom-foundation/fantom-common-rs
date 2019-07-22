use std::{
    cmp::PartialEq,
    error::Error as StdError,
    fmt::{self, Display},
    io,
};

/// The top-level result type for dealing with Offscale.io stack.
pub type Result<T> = std::result::Result<T, Error>;

/// An Error type encapsulating various issues that may come up
/// in both the expected and unexpected operation of the system.
#[derive(Debug)]
pub enum Error {
    /// Not an error; usually means option None when no error detected.
    NoneError,
    /// The system has been used in an unsupported way.
    Unsupported(String),
    /// An unexpected bug has happened. Please open an issue on github!
    ReportableBug(String),
    /// A read or write error has happened when interacting with the file system.
    Io(io::Error),
}

use self::Error::*;

impl Clone for Error {
    fn clone(&self) -> Error {
        match self {
            // Add here variants for new Error enum members when needed.
            NoneError => NoneError,
            Unsupported(why) => Unsupported(why.clone()),
            ReportableBug(what) => ReportableBug(what.clone()),
            Io(ioe) => Io(std::io::Error::new(ioe.kind(), format!("{:?}", ioe))),
        }
    }
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match *self {
            // Add here variants for new Error enum members when needed.
            NoneError => {
                if let NoneError = *other {
                    true
                } else {
                    false
                }
            }
            Unsupported(ref l) => {
                if let Unsupported(ref r) = *other {
                    l == r
                } else {
                    false
                }
            }
            ReportableBug(ref l) => {
                if let ReportableBug(ref r) = *other {
                    l == r
                } else {
                    false
                }
            }
            Io(_) => false,
        }
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(io_error: io::Error) -> Error {
        Error::Io(io_error)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            NoneError => "Not an error.",
            Unsupported(ref e) => &*e,
            ReportableBug(ref e) => &*e,
            Io(ref e) => e.description(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match *self {
            NoneError => write!(f, "Not an error."),
            Unsupported(ref e) => write!(f, "Unsupported: {}", e),
            ReportableBug(ref e) => write!(
                f,
                "Unexpected bug has happened: {}. \
                 PLEASE REPORT THIS BUG!",
                e
            ),
            Io(ref e) => write!(f, "IO error: {}", e),
        }
    }
}
