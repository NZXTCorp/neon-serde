//! Defines error handling types used by the create
//! uses the `error-chain` create for generation

extern crate neon_runtime;

use neon;
use serde::{de, ser};
use std::convert::From;
use std::fmt::Display;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

impl Error {
    #[allow(clippy::must_use_candidate)]
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

#[derive(Debug, ThisError)]
pub enum ErrorKind {
    /// nodejs has a hard coded limit on string length
    /// trying to serialize a string that is too long will result in an error
    #[error("String too long for nodejs len: {}", 0)]
    StringTooLong(usize),
    /// when deserializing to a boolean `false` `undefined` `null` `number`
    /// are valid inputs
    /// any other types will result in error-chain
    #[error("Unable to coerce value to type: {}", 0)]
    UnableToCoerce(&'static str),
    /// occurs when deserializing a char from an empty string
    #[error("EmptyString")]
    EmptyString,
    /// occurs when deserializing a char from a sting with
    /// more than one character
    #[error("String too long to be a char expected len: 1 got len: {}", 0)]
    StringTooLongForChar(usize),
    /// occurs when a deserializer expects a `null` or `undefined`
    /// property and found another type
    #[error("ExpectingNull")]
    ExpectingNull,
    /// occurs when deserializing to an enum and the source object has
    /// a none-1 number of properties
    #[error("key: '{}'", 0)]
    // TODO: add `InvalidKeyType` diagnostic
    InvalidKeyType(String),
    /// an internal deserialization error from an invalid array
    #[error("ArrayIndexOutOfBounds: attempt to access ({}) size: ({})", 0, 1)]
    ArrayIndexOutOfBounds(u32, u32),
    #[doc(hidden)]
    /// This type of object is not supported
    #[error("Not Implemented: '{}'", 0)]
    NotImplemented(&'static str),
    /// A JS exception was thrown
    #[error("JS exception")]
    Js(neon::result::Throw),
    // failed to convert something to f64
    #[error("CastError")]
    CastError,

    #[error("{}", 0)]
    Msg(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Msg(msg.to_string()).into()
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Msg(msg.to_string()).into()
    }
}

impl Error {
    fn into_throw<'a, 'b: 'a, C>(self, cx: &'a mut C) -> neon::result::Throw
    where
        C: neon::context::Context<'b>,
    {
        if let ErrorKind::Js(t) = self.0 {
            return t;
        };
        cx.throw_error::<String, ()>(format!("{:?}", self)).unwrap_err()
    }
}

pub trait MapErrIntoThrow<T> {
    #[allow(clippy::missing_errors_doc)] // Nothing useful to say about the error. It returns a converted error if one is contained.
    fn map_err_into_throw<'a, 'b: 'a, C>(self, cx: &'a mut C) -> neon::result::NeonResult<T>
    where
        C: neon::context::Context<'b>;
}

impl<T> MapErrIntoThrow<T> for std::result::Result<T, Error> {
    fn map_err_into_throw<'a, 'b: 'a, C>(self, cx: &'a mut C) -> neon::result::NeonResult<T>
where
    C: neon::context::Context<'b>
    {
        self.map_err(|e| e.into_throw(cx))
    }
}

impl From<neon::result::Throw> for Error {
    fn from(throw: neon::result::Throw) -> Self {
        ErrorKind::Js(throw).into()
    }
}
