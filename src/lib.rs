#![allow(unknown_lints)]
#![deny(unused_variables)]
#![deny(unused_mut)]
#![deny(clippy)]
#![deny(clippy_pedantic)]
#![allow(stutter)]
#![recursion_limit = "128"]

//!
//! Neon-serde
//! ==========
//!
//! This crate is a utility to easily convert values between
//!
//! A `Handle<JsValue>` from the `neon` crate
//! and any value implementing `serde::{Serialize, Deserialize}`
//!
//! ## Usage
//!
//! #### `neon_serde::from_value`
//! Convert a `Handle<js::JsValue>` to
//! a type implementing `serde::Deserialize`
//!
//! #### `neon_serde::to_value`
//! Convert a value implementing `serde::Serialize` to
//! a `Handle<JsValue>`
//!
//!
//! ## Example
//!
//! ```rust,no_run
//! # #![allow(dead_code)]
//! extern crate neon_serde;
//! extern crate neon;
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use neon::prelude::*;
//! use neon_serde::errors::MapErrIntoThrow;
//!
//! #[derive(Serialize, Debug, Deserialize)]
//! struct AnObject {
//!     a: u32,
//!     b: Vec<f64>,
//!     c: String,
//! }
//!
//! fn deserialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
//!     let arg0 = cx.argument::<JsValue>(0)?;
//!
//!     let arg0_value :AnObject = neon_serde::from_value(&mut cx, arg0).map_err_into_throw(&mut cx)?;
//!     println!("{:?}", arg0_value);
//!
//!     Ok(JsUndefined::new(&mut cx).upcast())
//! }
//!
//! fn serialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
//!     let value = AnObject {
//!         a: 1,
//!         b: vec![2f64, 3f64, 4f64],
//!         c: "a string".into()
//!     };
//!
//!     let js_value = neon_serde::to_value(&mut cx, &value).map_err_into_throw(&mut cx)?;
//!     Ok(js_value)
//! }
//!
//! # fn main () {
//! # }
//!
//! ```
//!

#[macro_use]
extern crate error_chain;
extern crate neon;
extern crate num;
#[macro_use]
extern crate serde;

pub mod de;
pub mod errors;
pub mod ser;

mod macros;

pub use de::from_value;
pub use de::from_value_opt;
pub use errors::MapErrIntoThrow;
pub use ser::to_value;

///
#[doc = include_str!("../readme.md")]
///
/// NOTE This private method is just so we include the examples from the 'readme.md' in the doc-test
/// pass to make sure they still compile.
#[allow(dead_code)]
fn check_readme() {}

#[cfg(test)]
mod tests {
    use super::*;
    use neon::prelude::*;

    #[test]
    fn test_it_compiles() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> JsResult<'j, JsValue> {
            let result: () = {
                let arg: Handle<'j, JsValue> = cx.argument::<JsValue>(0)?;
                let () = from_value(&mut cx, arg).map_err_into_throw(&mut cx)?;
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result).map_err_into_throw(&mut cx)?;
            Ok(result)
        }

        let _ = check;
    }

    #[test]
    fn test_it_compiles_2() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> JsResult<'j, JsValue> {
            let result: () = {
                let arg: Option<Handle<'j, JsValue>> = cx.argument_opt(0);
                let () = from_value_opt(&mut cx, arg).map_err_into_throw(&mut cx)?;
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result).map_err_into_throw(&mut cx)?;
            Ok(result)
        }

        let _ = check;
    }
}
