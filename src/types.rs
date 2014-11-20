//! Data types used in our public API.

use ffi::Encoding;

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
    pub fn from_bool(is_reliable: bool) -> Reliable {
        if is_reliable { Reliable::Yes } else { Reliable::No }
    }
}

/// A language code, normally two letters for common languages.
#[deriving(PartialEq, Eq, Show)]
pub struct LanguageCode(pub &'static str);

#[deriving(Show, Default)]
pub struct Hints<'a> {
    pub content_language: Option<&'a str>,
    pub tld: Option<&'a str>,
    pub encoding: Option<Encoding>,
    pub language: Option<LanguageCode>
}

pub struct LanguageScore {
    pub language: Option<LanguageCode>,
    pub percent: i32,
    pub normalized_score: f64
}

pub struct DetectionResult {
    pub language: Option<LanguageCode>,
    pub scores: [LanguageScore, ..3],
    pub text_bytes: i32,
    pub reliable: Reliable
}
