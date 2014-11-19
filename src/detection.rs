//! Interfaces to the detector itself.

use libc::c_int;
use ffi::CLD2_DetectLanguage;
use language::{LanguageId, LanguageIdExt};

/// Possible data formats.
#[deriving(PartialEq, Eq, Show)]
pub enum Format {
    /// Process the text as-is.
    Text, 
    /// Try to stip HTML tags and expand entities.
    Html
}

/// Is the output of the language decoder reliable?
#[deriving(PartialEq, Eq, Show)]
pub enum Reliable {
    /// The guess is probably good.
    Yes, 
    /// This value is not especially trustworthy.
    No
}

impl Reliable {
    /// Construct from a boolean value.
    fn from_bool(is_reliable: bool) -> Reliable {
        if is_reliable { Reliable::Yes } else { Reliable::No }
    }
}

/// A language code, normally two letters for common languages.
#[deriving(PartialEq, Eq, Show)]
pub struct LanguageCode(pub &'static str);

impl LanguageCode {
    /// Construct a language code from a LanguageId.
    fn from_id(id: LanguageId) -> Option<LanguageCode> {
        if id.is_known() {
            Some(LanguageCode(id.code()))
        } else {
            None
        }
    }
}

/// Detect the language of the input text.
///
/// ```
/// use cld2::{detect_language, Format, Reliable, LanguageCode};
///
/// let text = "
/// It is an ancient Mariner,
/// And he stoppeth one of three.
/// 'By thy long grey beard and glittering eye,
/// Now wherefore stopp'st thou me?
/// ";
/// assert_eq!((Some(LanguageCode("en")), Reliable::Yes),
///            detect_language(text, Format::Text));
///
/// assert_eq!((Some(LanguageCode("en")), Reliable::No),
///            detect_language("blah", Format::Html));
/// ```
pub fn detect_language(text: &str, format: Format) ->
    (Option<LanguageCode>, Reliable)
{
    let mut is_reliable: bool = false;
    let lang_id = unsafe {
        CLD2_DetectLanguage(text.as_ptr() as *const i8, text.len() as c_int,
                            format == Format::Text, &mut is_reliable)
    };
    (LanguageCode::from_id(lang_id), Reliable::from_bool(is_reliable))
}
