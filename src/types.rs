//! Data types used in our public API.

use ffi::Encoding;
pub use self::Reliability::{Reliable, Unreliable};

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
pub enum Reliability {
    /// The decoder is reasonably confident about this guess.
    Reliable,
    /// The decoder does not have high confidence in this guess.
    Unreliable
}

impl Reliability {
    /// Construct from a boolean value.
    pub fn from_bool(is_reliable: bool) -> Reliability {
        if is_reliable { Reliable } else { Unreliable }
    }
}

/// A language code, normally two letters for common languages.
#[deriving(PartialEq, Eq, Show)]
pub struct Lang(pub &'static str);

#[deriving(Show, Default)]
pub struct Hints<'a> {
    pub content_language: Option<&'a str>,
    pub tld: Option<&'a str>,
    pub encoding: Option<Encoding>,
    pub language: Option<Lang>
}

pub struct LanguageScore {
    pub language: Option<Lang>,
    pub percent: i32,
    pub normalized_score: f64
}

pub struct DetectionResult {
    pub language: Option<Lang>,
    pub scores: [LanguageScore, ..3],
    pub text_bytes: i32,
    pub reliable: Reliability
}
