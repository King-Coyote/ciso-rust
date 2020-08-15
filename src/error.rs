use std::error::Error as StdError;
use std::fmt::{Debug};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidArgs(String), // you provided inmvalid args to a rust-to-lua function
    FunctionNotFound(String), // a function you tried to call from lua was not found
}

impl fmt::Display for Error {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidArgs(ref reason) => {
                write!(f, "{}", reason)
            },
            Error::FunctionNotFound(ref func_name) => {
                write!(f, "Function with name {} not found.", func_name)
            }
        }
    }
}

impl StdError for Error {}