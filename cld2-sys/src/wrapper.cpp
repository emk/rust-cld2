#include "wrapper.h"

Language CLD2_DetectLanguage(const char* buffer,
                             int buffer_length,
                             bool is_plain_text,
                             bool* is_reliable)
{
    return CLD2::DetectLanguage(buffer, buffer_length, is_plain_text,
                                is_reliable);
}
