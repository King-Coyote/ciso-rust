use std::error::Error as StdError;
use std::fmt::{Debug};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    // you provided inmvalid args to a rust-to-lua function
    InvalidArgs(String)
}

impl fmt::Display for Error {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidArgs(ref reason) => {
                write!(f, "{}", reason)
            },
        }
    }
}

impl StdError for Error {}