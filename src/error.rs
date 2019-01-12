//! Error handling for the linalg module.

use alloc::boxed::Box;
use core::convert::Into;
// use core::error;
use core::fmt;
use core::marker::{Send, Sync};

/// An error related to the linalg module.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: Box<&'static str>,
}

/// Types of errors produced in the linalg module.
///
/// List intended to grow and so you should
/// be wary of matching against explicitly.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    /// An argument did not uphold a necessary criteria for the function.
    InvalidArg,
    /// A failure to decompose due to some property of the data.
    DecompFailure,
    /// A failure due to some algebraic constraints not being met.
    AlgebraFailure,
    /// Tried to divide by zero
    DivByZero,
    /// Failure due to inability to convert between scalar types
    ScalarConversionFailure,
    /// A user-supplied permutation is not a valid permutation.
    InvalidPermutation
}

impl Error {
    /// Construct a new `Error` of a particular `ErrorKind`.
    pub fn new<E>(kind: ErrorKind, error: &'static str) -> Error
        where E: Into<Box<Send + Sync>>
    {
        Error {
            kind: kind,
            error: Box::new(error),
        }
    }

    /// Get the kind of this `Error`.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    fn description(&self) -> &str {
        *self.error
    }
}

// impl error::Error for Error {
//     fn description(&self) -> &str {
//         self.error.description()
//     }
// }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}
