//! Detect the language of a string using the [cld2 library][cld2] from the
//! Chromium project.
//!
//! ```
//! use cld2::{detect_language, Format, Reliable, Unreliable, Lang};
//!
//! let text = "It is an ancient Mariner,
//! And he stoppeth one of three.
//! 'By thy long grey beard and glittering eye,
//! Now wherefore stopp'st thou me?";
//!
//! assert_eq!((Some(Lang("en")), Reliable),
//!            detect_language(text, Format::Text));
//! ```
//!
//! This library wraps the `cld2-sys` library, which provides a raw
//! interface to cld2.  The only major feature which isn't yet wrapped is
//! the `ResultChunk` interface, because it tends to give fairly imprecise
//! answers&mdash;it wouldn't make a very good multi-lingual spellchecker
//! component, for example.  As always, pull requests are eagerly welcome!
//!
//! For more information, see the [GitHub project][github] for this
//! library.
//!
//! [cld2]: https://code.google.com/p/cld2/
//! [github]: https://github.com/emk/rust-cld2

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
