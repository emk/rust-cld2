//! A semi-reasonable interface to the language IDs used by cld2.  We could
//! theoretically export this if somebody needed it.

#![allow(missing_docs)]
#![experimental]

use std::str::raw::c_str_to_static_slice;
use ffi::{CLD2_GetLanguageFromName, CLD2_LanguageName,
          CLD2_LanguageCode, CLD2_LanguageDeclaredName};
pub use ffi::Language as LanguageId;

use types::Lang;

pub trait LanguageIdExt {
    fn from_name(name: &str) -> Self;
    fn name(&self) -> &'static str;
    fn code(&self) -> &'static str;
    fn declared_name(&self) -> &'static str;
    fn is_unknown(&self) -> bool;
    fn is_known(&self) -> bool;
    fn to_lang(&self) -> Option<Lang>;
}

impl LanguageIdExt for LanguageId {
    fn from_name(name: &str) -> LanguageId {
        unsafe { name.with_c_str(|name| CLD2_GetLanguageFromName(name)) }
    }

    fn name(&self) -> &'static str {
        unsafe { c_str_to_static_slice(CLD2_LanguageName(*self)) }
    }

    fn code(&self) -> &'static str {
        unsafe { c_str_to_static_slice(CLD2_LanguageCode(*self)) }
    }

    fn declared_name(&self) -> &'static str {
        unsafe { c_str_to_static_slice(CLD2_LanguageDeclaredName(*self)) }
    }

    fn is_unknown(&self) -> bool { *self == LanguageId::UNKNOWN_LANGUAGE }
    fn is_known(&self) -> bool { *self != LanguageId::UNKNOWN_LANGUAGE }

    fn to_lang(&self) -> Option<Lang> {
        if self.is_known() {
            Some(Lang(self.code()))
        } else {
            None
        }
    }
}

#[test]
fn test_language_ext() {
    let lang = LanguageIdExt::from_name("en");
    assert_eq!(LanguageId::ENGLISH, lang);
    assert_eq!("ENGLISH", lang.name());
    assert_eq!("en", lang.code());
    assert_eq!("ENGLISH", lang.declared_name());

    assert_eq!(true, LanguageId::ENGLISH.is_known());
    assert_eq!(false, LanguageId::ENGLISH.is_unknown());
    assert_eq!(false, LanguageId::UNKNOWN_LANGUAGE.is_known());
    assert_eq!(true, LanguageId::UNKNOWN_LANGUAGE.is_unknown());

    // This is a bit yucky.
    assert_eq!("un", LanguageId::UNKNOWN_LANGUAGE.code());
}
