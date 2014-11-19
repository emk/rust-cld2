/* Originally generated by rust-bindgen, then cleaned up by hand. */

use libc::{c_char, c_int, c_double, c_void};
use libc::types::os::arch::c95::size_t;
use libc::types::common::c99::uint16_t;

use languages::Language;

#[repr(C)]
pub struct CLDHints {
    pub content_language_hint: *const c_char,
    pub tld_hint: *const c_char,
    pub encoding_hint: c_int,
    pub language_hint: Language,
}

#[deriving(Show)]
#[repr(C)]
pub struct ResultChunk {
    pub offset: c_int,
    pub bytes: uint16_t,
    pub lang1: uint16_t,
}

pub type ResultChunks = c_void;

extern "C" {
    pub fn CLD2_ResultChunkVector_new() -> *mut ResultChunks;
    pub fn CLD2_ResultChunkVector_data(chunks: *const ResultChunks)
     -> *const ResultChunk;
    pub fn CLD2_ResultChunkVector_size(chunks: *const ResultChunks) -> size_t;
    pub fn CLD2_ResultChunkVector_delete(chunks: *mut ResultChunks);

    pub fn CLD2_DetectLanguage(buffer: *const c_char,
                               buffer_length: c_int,
                               is_plain_text: bool, is_reliable: *mut bool)
     -> Language;
    pub fn CLD2_DetectLanguageSummary(buffer: *const c_char,
                                      buffer_length: c_int,
                                      is_plain_text: bool,
                                      language3: *mut Language,
                                      percent3: *mut c_int,
                                      text_bytes: *mut c_int,
                                      is_reliable: *mut bool) -> Language;
    pub fn CLD2_DetectLanguageSummary2(buffer: *const c_char,
                                       buffer_length: c_int,
                                       is_plain_text: bool,
                                       tld_hint: *const c_char,
                                       encoding_hint: c_int,
                                       language_hint: Language,
                                       language3: *mut Language,
                                       percent3: *mut c_int,
                                       text_bytes: *mut c_int,
                                       is_reliable: *mut bool) -> Language;
    pub fn CLD2_ExtDetectLanguageSummary(buffer: *const c_char,
                                         buffer_length: c_int,
                                         is_plain_text: bool,
                                         language3: *mut Language,
                                         percent3: *mut c_int,
                                         text_bytes: *mut c_int,
                                         is_reliable: *mut bool) -> Language;
    pub fn CLD2_ExtDetectLanguageSummary2(buffer: *const c_char,
                                          buffer_length: c_int,
                                          is_plain_text: bool,
                                          tld_hint: *const c_char,
                                          encoding_hint: c_int,
                                          language_hint: Language,
                                          language3: *mut Language,
                                          percent3: *mut c_int,
                                          text_bytes: *mut c_int,
                                          is_reliable: *mut bool)
     -> Language;
    pub fn CLD2_ExtDetectLanguageSummary3(buffer: *const c_char,
                                          buffer_length: c_int,
                                          is_plain_text: bool,
                                          tld_hint: *const c_char,
                                          encoding_hint: c_int,
                                          language_hint: Language,
                                          language3: *mut Language,
                                          percent3: *mut c_int,
                                          normalized_score3:
                                              *mut c_double,
                                          text_bytes: *mut c_int,
                                          is_reliable: *mut bool)
     -> Language;
    pub fn CLD2_ExtDetectLanguageSummary4(buffer: *const c_char,
                                          buffer_length: c_int,
                                          is_plain_text: bool,
                                          cld_hints: *const CLDHints,
                                          flags: c_int,
                                          language3: *mut Language,
                                          percent3: *mut c_int,
                                          normalized_score3:
                                              *mut c_double,
                                          resultchunkvector: *mut ResultChunks,
                                          text_bytes: *mut c_int,
                                          is_reliable: *mut bool)
     -> Language;
    pub fn CLD2_DetectLanguageVersion() -> *const c_char;
}
