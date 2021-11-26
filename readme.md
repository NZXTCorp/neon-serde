Neon-serde
==========

[![Build Status](https://travis-ci.org/GabrielCastro/neon-serde.svg?branch=master)](https://travis-ci.org/GabrielCastro/neon-serde)
[![](https://meritbadge.herokuapp.com/neon-serde)](https://crates.io/crates/neon-serde)

This crate is a utility to easily convert values between

A `Handle<JsValue>` from the [neon](https://github.com/neon-bindings/neon) crate
and any value implementing `serde::{Serialize, Deserialize}`

## Versions support

neon-serde is tested on node
`8` `10` `12`

## Usage

#### `neon_serde::from_value`
Convert a `Handle<js::JsValue>` to
a type implementing `serde::Deserialize`

#### `neon_serde::to_value`˚
Convert a value implementing `serde::Serialize` to
a `Handle<JsValue>`

## Export Macro example
The `export!` macro allows you to quickly define functions automatically convert their arguments

```rust,no_run
#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;
use neon_serde::errors::MapErrIntoThrow;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u16,
}

export! {

    /// Say hello based on a persons name
    fn say_hello(name: String) -> String {
        format!("Hello, {}!", name)
    }

    /// Say how old someone is
    fn greet(user: User) -> String {
        format!("{} is {} years old", user.name, user.age)
    }

    /// Say how old someone is, if they exist
    fn maybe_say_hello(user: Option<User>) -> Option<String> {
        user.map(greet)
    }

    /// using `Vec<u8>` not accept a buffer
    fn expect_array(_buff: Vec<u8>) -> () {
        // code
    }

    /// calculate fibonacci recursively
    fn fibonacci(n: i32) -> i32 {
        match n {
            1 | 2 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}
# // This is needed for when rustdoc parses this as a doc-test
# fn main () {}
```

The `export!` macro with `serde_bytes`:

```rust,ignore
#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
use neon_serde::errors::MapErrIntoThrow;

export! {
    /// Sorts the bytes in a string
    /// use `serde_bytes::ByteBuf` to return a `Buffer` in node
    /// a `Vec<u8>` will be an array
    fn sort_utf8_bytes(str: String) -> serde_bytes::ByteBuf {
        let mut bytes = str.into_bytes();
        bytes.sort();
        serde_bytes::ByteBuf::from(bytes)
    }

    /// using `serde_bytes::ByteBuf` will make passing an array
    /// of numbers an error
    ///
    /// note: `-> ()` is NOT optional
    fn expect_buffer_only(_buff: serde_bytes::ByteBuf) -> () {
        // code
    }
}
```

## Direct Usage Example

```rust,no_run
extern crate neon_serde;
extern crate neon;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;
use neon_serde::errors::MapErrIntoThrow;

#[derive(Serialize, Debug, Deserialize)]
struct AnObject {
    a: u32,
    b: Vec<f64>,
    c: String,
}

fn deserialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg0 = cx.argument::<JsValue>(0)?;

    let arg0_value :AnObject = neon_serde::from_value(&mut cx, arg0).map_err_into_throw(&mut cx)?;
    println!("{:?}", arg0_value);

    Ok(JsUndefined::new(&mut cx).upcast())
}

fn serialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let value = AnObject {
        a: 1,
        b: vec![2f64, 3f64, 4f64],
        c: "a string".into()
    };

    let js_value = neon_serde::to_value(&mut cx, &value).map_err_into_throw(&mut cx)?;
    Ok(js_value)
}
# // This is needed for when rustdoc parses this as a doc-test
# fn main () {}
```

## Limitations

### Data ownership
All Deserialize Values must own all their data (they must have the trait `serde::DererializeOwned`)
