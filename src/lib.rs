//! Detect the language of a string using the cld2 library from the
//! Chromium project.

#![license = "Public domain (Unlicense)"]
#![unstable]
#![warn(missing_docs)]
#![feature(globs)]

extern crate libc;
extern crate "cld2-sys" as ffi;

pub use detection::*;

mod language;
mod detection;
