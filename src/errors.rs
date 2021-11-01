//! Defines error handling types used by the create
//! uses the `error-chain` create for generation

extern crate neon_runtime;


use neon;
use serde::{de, ser};
use std::convert::From;
use std::fmt::Display;

error_chain! {
    errors {
        /// nodejs has a hard coded limit on string length
        /// trying to serialize a string that is too long will result in an error
        StringTooLong(len: usize) {
            description("String too long for nodejs")
            display("String too long for nodejs len: {}", len)
        }
        /// when deserializing to a boolean `false` `undefined` `null` `number`
        /// are valid inputs
        /// any other types will result in error
        UnableToCoerce(to_type: &'static str) {
            description("Unable to coerce")
            display("Unable to coerce value to type: {}", to_type)
        }
        /// occurs when deserializing a char from an empty string
        EmptyString {
            description("EmptyString")
            display("EmptyString")
        }
        /// occurs when deserializing a char from a sting with
        /// more than one character
        StringTooLongForChar(len: usize) {
            description("String too long to be a char")
            display("String too long to be a char expected len: 1 got len: {}", len)
        }
        /// occurs when a deserializer expects a `null` or `undefined`
        /// property and found another type
        ExpectingNull {
            description("ExpectingNull")
            display("ExpectingNull")
        }
        /// occurs when deserializing to an enum and the source object has
        /// a none-1 number of properties
        InvalidKeyType(key: String) {
            description("InvalidKeyType")
            display("key: '{}'", key)
        }
        /// an internal deserialization error from an invalid array
        ArrayIndexOutOfBounds(index: u32, length: u32) {
            description("ArrayIndexOutOfBounds")
            display(
                "ArrayIndexOutOfBounds: attempt to access ({}) size: ({})",
                index,
                length
            )
        } #[doc(hidden)]
        /// This type of object is not supported
        NotImplemented(name: &'static str) {
            description("Not Implemented")
            display("Not Implemented: '{}'", name)
        }
        /// A JS exception was thrown
        Js(throw: neon::result::Throw) {
            description("JS exception")
            display("JS exception")
        }
        // failed to convert something to f64
        CastError {
            description("CastError")
            display("CastError")
        }
    }
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
        if let ErrorKind::Js(_) = *self.kind() {
            return neon::result::Throw;
        };
        cx.throw_error::<String, ()>(format!("{:?}", self)).unwrap_err()
    }
}

pub trait MapErrIntoThrow<T> {
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
