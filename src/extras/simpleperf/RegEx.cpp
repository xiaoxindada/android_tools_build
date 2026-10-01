/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "RegEx.h"

#include <android-base/logging.h>
#include <pcre2.h>

// PCRE2 is used for regex implementation because it provides a good balance between
// search efficiency and binary size. In a local experiment searching process maps:
//   std::regex: 5986 ms
//   re2: 40 ms
//   pcre2: 211 ms
//   pcre2 (with preallocated match data): 96 ms
//
// Binary size comparison (simpleperf):
//   with std::regex: 3,191,856 bytes
//   with re2: 3,640,936 bytes
//   with pcre2 (static): 3,452,896 bytes
//   with pcre2 (shared): 3,140,656 bytes
//
// PCRE2 with preallocated match data is significantly faster than std::regex while
// keeping a smaller binary size than re2.

namespace simpleperf {

RegExMatch::~RegExMatch() {}

class RegExMatchImpl : public RegExMatch {
 public:
  RegExMatchImpl(std::string_view s, pcre2_code* re) : s_(s), re_(re) {
    match_data_ = pcre2_match_data_create_from_pattern(re_, nullptr);
    current_offset_ = 0;
    MoveToNextMatch();
  }

  ~RegExMatchImpl() override { pcre2_match_data_free(match_data_); }

  bool IsValid() const override { return is_valid_; }

  std::string GetField(size_t index) const override {
    if (!is_valid_) return "";
    uint32_t count = pcre2_get_ovector_count(match_data_);
    if (index >= count) return "";
    PCRE2_SIZE* ovector = pcre2_get_ovector_pointer(match_data_);
    if (ovector[2 * index] == PCRE2_UNSET) return "";
    return std::string(s_.substr(ovector[2 * index], ovector[2 * index + 1] - ovector[2 * index]));
  }

  void MoveToNextMatch() override {
    int rc = pcre2_match(re_, reinterpret_cast<PCRE2_SPTR>(s_.data()), s_.size(), current_offset_,
                         0, match_data_, nullptr);
    if (rc >= 0) {
      is_valid_ = true;
      PCRE2_SIZE* ovector = pcre2_get_ovector_pointer(match_data_);
      PCRE2_SIZE match_end = ovector[1];
      if (match_end == current_offset_) {
        current_offset_++;
      } else {
        current_offset_ = match_end;
      }
    } else {
      is_valid_ = false;
    }
  }

 private:
  std::string_view s_;
  pcre2_code* re_;
  pcre2_match_data* match_data_;
  PCRE2_SIZE current_offset_;
  bool is_valid_ = false;
};

class RegExImpl : public RegEx {
 public:
  RegExImpl(std::string_view pattern, pcre2_code* re) : RegEx(pattern), re_(re) {
    match_data_ = pcre2_match_data_create_from_pattern(re_, nullptr);
  }

  ~RegExImpl() override {
    pcre2_match_data_free(match_data_);
    pcre2_code_free(re_);
  }

  bool Match(std::string_view s) const override {
    pcre2_match_data* match_data = pcre2_match_data_create_from_pattern(re_, nullptr);
    int rc = pcre2_match(re_, reinterpret_cast<PCRE2_SPTR>(s.data()), s.size(), 0,
                         PCRE2_ANCHORED | PCRE2_ENDANCHORED, match_data, nullptr);
    pcre2_match_data_free(match_data);
    return rc >= 0;
  }

  bool Search(std::string_view s) const override {
    pcre2_match_data* match_data = pcre2_match_data_create_from_pattern(re_, nullptr);
    int rc = pcre2_match(re_, reinterpret_cast<PCRE2_SPTR>(s.data()), s.size(), 0, 0, match_data,
                         nullptr);
    pcre2_match_data_free(match_data);
    return rc >= 0;
  }

  bool ThreadUnsafeMatch(std::string_view s) const override {
    int rc = pcre2_match(re_, reinterpret_cast<PCRE2_SPTR>(s.data()), s.size(), 0,
                         PCRE2_ANCHORED | PCRE2_ENDANCHORED, match_data_, nullptr);
    return rc >= 0;
  }

  bool ThreadUnsafeSearch(std::string_view s) const override {
    int rc = pcre2_match(re_, reinterpret_cast<PCRE2_SPTR>(s.data()), s.size(), 0, 0, match_data_,
                         nullptr);
    return rc >= 0;
  }

  std::unique_ptr<RegExMatch> SearchAll(std::string_view s) const override {
    return std::make_unique<RegExMatchImpl>(s, re_);
  }

  std::optional<std::string> Replace(const std::string& s,
                                     const std::string& format) const override {
    PCRE2_SIZE outlen = s.size() * 2 + 128;
    std::string result(outlen, '\0');
    int rc = pcre2_substitute(re_, reinterpret_cast<PCRE2_SPTR>(s.data()), s.size(), 0,
                              PCRE2_SUBSTITUTE_GLOBAL | PCRE2_SUBSTITUTE_EXTENDED, nullptr, nullptr,
                              reinterpret_cast<PCRE2_SPTR>(format.data()), format.size(),
                              reinterpret_cast<PCRE2_UCHAR*>(result.data()), &outlen);
    if (rc == PCRE2_ERROR_NOMEMORY) {
      result.resize(outlen);
      rc = pcre2_substitute(re_, reinterpret_cast<PCRE2_SPTR>(s.data()), s.size(), 0,
                            PCRE2_SUBSTITUTE_GLOBAL | PCRE2_SUBSTITUTE_EXTENDED, nullptr, nullptr,
                            reinterpret_cast<PCRE2_SPTR>(format.data()), format.size(),
                            reinterpret_cast<PCRE2_UCHAR*>(result.data()), &outlen);
    }
    if (rc >= 0) {
      result.resize(outlen);
      return result;
    }
    return std::nullopt;
  }

 private:
  pcre2_code* re_;
  pcre2_match_data* match_data_;
};

std::unique_ptr<RegEx> RegEx::Create(std::string_view pattern) {
  int errornumber;
  PCRE2_SIZE erroroffset;
  pcre2_code* re = pcre2_compile(reinterpret_cast<PCRE2_SPTR>(pattern.data()), pattern.size(), 0,
                                 &errornumber, &erroroffset, nullptr);
  if (re == nullptr) {
    PCRE2_UCHAR buffer[256];
    pcre2_get_error_message(errornumber, buffer, sizeof(buffer));
    LOG(ERROR) << "regex error: " << buffer << " at offset " << erroroffset << ", pattern "
               << pattern;
    return nullptr;
  }
  return std::make_unique<RegExImpl>(pattern, re);
}

bool SearchInRegs(std::string_view s, const std::vector<std::unique_ptr<RegEx>>& regs) {
  for (auto& reg : regs) {
    if (reg->Search(s)) {
      return true;
    }
  }
  return false;
}

}  // namespace simpleperf
