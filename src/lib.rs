//! Detect the language of a string using the cld2 library from the
//! Chromium project.

#![license = "Public domain (Unlicense)"]
#![unstable]
#![deny(missing_docs)]
#![feature(globs)]

extern crate libc;
extern crate "cld2-sys" as ffi;

pub use types::*;
pub use detection::*;

mod types;
mod language;
mod detection;
