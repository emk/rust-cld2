#include "wrapper.h"

Language CLD2_DetectLanguage(const char* buffer,
                             int buffer_length,
                             bool is_plain_text,
                             bool* is_reliable)
{
    return CLD2::DetectLanguage(buffer, buffer_length, is_plain_text,
                                is_reliable);
}

Language CLD2_DetectLanguageSummary(const char* buffer,
                                    int buffer_length,
                                    bool is_plain_text,
                                    Language* language3,
                                    int* percent3,
                                    int* text_bytes,
                                    bool* is_reliable)
{
    return CLD2::DetectLanguageSummary(buffer, buffer_length, is_plain_text,
                                       language3, percent3, text_bytes,
                                       is_reliable);
}

Language CLD2_DetectLanguageSummary2(const char* buffer,
                                     int buffer_length,
                                     bool is_plain_text,
                                     const char* tld_hint,
                                     int encoding_hint,
                                     Language language_hint,
                                     Language* language3,
                                     int* percent3,
                                     int* text_bytes,
                                     bool* is_reliable)
{
    return CLD2::DetectLanguageSummary(buffer, buffer_length, is_plain_text,
                                       tld_hint, encoding_hint, language_hint,
                                       language3, percent3, text_bytes,
                                       is_reliable);
}

Language CLD2_ExtDetectLanguageSummary(const char* buffer,
                                       int buffer_length,
                                       bool is_plain_text,
                                       Language* language3,
                                       int* percent3,
                                       int* text_bytes,
                                       bool* is_reliable)
{
    return CLD2::ExtDetectLanguageSummary(buffer, buffer_length, is_plain_text,
                                          language3, percent3, text_bytes,
                                          is_reliable);
}

Language CLD2_ExtDetectLanguageSummary2(const char* buffer,
                                        int buffer_length,
                                        bool is_plain_text,
                                        const char* tld_hint,
                                        int encoding_hint,
                                        Language language_hint,
                                        Language* language3,
                                        int* percent3,
                                        int* text_bytes,
                                        bool* is_reliable)
{
    return CLD2::ExtDetectLanguageSummary(buffer, buffer_length, is_plain_text,
                                          tld_hint, encoding_hint,
                                          language_hint, language3, percent3,
                                          text_bytes, is_reliable);
}

Language CLD2_ExtDetectLanguageSummary3(const char* buffer,
                                        int buffer_length,
                                        bool is_plain_text,
                                        const char* tld_hint,
                                        int encoding_hint,
                                        Language language_hint,
                                        Language* language3,
                                        int* percent3,
                                        double* normalized_score3,
                                        int* text_bytes,
                                        bool* is_reliable)
{
    return CLD2::ExtDetectLanguageSummary(buffer, buffer_length, is_plain_text,
                                          tld_hint, encoding_hint, language_hint,
                                          language3, percent3, normalized_score3,
                                          text_bytes, is_reliable);
}


  /* Needs a custom C wrapper to handle std::vector.
  Language CLD2_ExtDetectLanguageSummary4(
                          const char* buffer,
                          int buffer_length,
                          bool is_plain_text,
                          const CLDHints* cld_hints,
                          int flags,
                          Language* language3,
                          int* percent3,
                          double* normalized_score3,
                          ResultChunkVector* resultchunkvector,
                          int* text_bytes,
                          bool* is_reliable);
  */

const char* CLD2_DetectLanguageVersion() {
    return CLD2::DetectLanguageVersion();
}
