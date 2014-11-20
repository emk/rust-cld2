//! Interfaces to the detector itself.

use std::default::Default;
use std::ptr::{null, null_mut};

use libc::{c_int, c_double};
use ffi::{CLD2_DetectLanguage, CLDHints, Encoding,
          CLD2_ExtDetectLanguageSummary4};
use ffi::Language as LanguageId;

use language::LanguageIdExt;
use types::*;

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
/// assert_eq!((None, Reliable::No),
///            detect_language("blah", Format::Html));
/// ```
pub fn detect_language(text: &str, format: Format) ->
    (Option<LanguageCode>, Reliable)
{
    let result = detect_language_summary(text, format, &Default::default());
    (result.language, result.reliable)
}

pub fn detect_language_summary(text: &str, format: Format, hints: &Hints)
    -> DetectionResult
{
    let mut language3 = [LanguageId::UNKNOWN_LANGUAGE,
                         LanguageId::UNKNOWN_LANGUAGE,
                         LanguageId::UNKNOWN_LANGUAGE];
    let mut percent3: [c_int, ..3] = [0, 0, 0];
    let mut normalized_score3: [c_double, ..3] = [0.0, 0.0, 0.0];
    let mut text_bytes: c_int = 0;
    let mut is_reliable: bool = false;

    unsafe {
        hints.with_c_rep(|hints_ptr| {
            let lang = CLD2_ExtDetectLanguageSummary4(
                text.as_ptr() as *const i8, text.len() as c_int,
                format == Format::Text, hints_ptr, 0,
                language3.as_mut_slice().as_mut_ptr(),
                percent3.as_mut_slice().as_mut_ptr(),
                normalized_score3.as_mut_slice().as_mut_ptr(),
                null_mut(), &mut text_bytes, &mut is_reliable);
            from_ffi(lang, &language3, &percent3, &normalized_score3,
                     text_bytes, is_reliable)
        })
    }
}

trait WithCRep<R> {
    fn with_c_rep<T>(&self, body: |R| -> T) -> T;
}

impl<'a> WithCRep<*const CLDHints> for Hints<'a> {
    fn with_c_rep<T>(&self, body: |*const CLDHints| -> T) -> T {
        let clang = self.content_language.map(|v| v.to_c_str());
        let clang_ptr = clang.map(|v| v.as_ptr()).unwrap_or(null());
        let tld = self.tld.map(|v| v.to_c_str());
        let tld_ptr = tld.map(|v| v.as_ptr()).unwrap_or(null());
        let lang = self.language
            .map(|LanguageCode(c)| LanguageIdExt::from_name(c))
            .unwrap_or(LanguageId::UNKNOWN_LANGUAGE);
        let encoding = self.encoding
            .unwrap_or(Encoding::UNKNOWN_ENCODING) as c_int;
        let hints =
            CLDHints{content_language_hint: clang_ptr, tld_hint: tld_ptr,
                     encoding_hint: encoding, language_hint: lang};
        body(&hints)
    }
}

fn from_ffi(lang: LanguageId, language3: &[LanguageId, ..3],
            percent3: &[c_int, ..3], normalized_score3: &[c_double, ..3],
            text_bytes: c_int, reliable: bool) -> DetectionResult
{
    let score_n = |n| {
        LanguageScore{language: language3[n].to_lang(),
                      percent: percent3[n],
                      normalized_score: normalized_score3[n]}
    };

    DetectionResult{language: lang.to_lang(),
                    scores: [score_n(0), score_n(1), score_n(2)],
                    text_bytes: text_bytes,
                    reliable: Reliable::from_bool(reliable)}
}
