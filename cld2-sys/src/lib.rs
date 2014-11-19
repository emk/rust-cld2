//! Unsafe, low-level wrapper around cld2, the "compact language detector"
//! based on Chromium's code, plus a very thin C wrapper layer.  Normally
//! you won't want to use this library directly unless you're writing
//! your own cld2 wrapper library.
//!
//! If you need access to APIs which are not currently wrapped, please feel
//! free to send pull requests!

#![feature(globs)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;

pub use encodings::*;
pub use languages::*;
pub use flags::*;
pub use wrapper::*;

mod encodings;
mod languages;
mod flags;
mod wrapper;

// Just a single placeholder test in case somebody runs 'cargo test' in
// this library's directory, and not in the main library's directory.  This
// is not intended to be comprehensive, but please add regression tests for
// any bugs.
#[test]
fn test_detection() {
    let english = "
It is an ancient Mariner,
And he stoppeth one of three.
'By thy long grey beard and glittering eye,
Now wherefore stopp'st thou me?

'The Bridegroom's doors are opened wide,
And I am next of kin;
The guests are met, the feast is set:
May'st hear the merry din.'

He holds him with his skinny hand,
'There was a ship,' quoth he.
'Hold off! unhand me, grey-beard loon!'
Eftsoons his hand dropt he.
";

    let mut is_reliable: bool = false;
    let language = unsafe {
        CLD2_DetectLanguage(english.as_ptr() as *const i8,
                            english.len() as libc::c_int,
                            true, &mut is_reliable)
    };
    assert_eq!(Language::ENGLISH, language);
    assert_eq!(true, is_reliable);
}
