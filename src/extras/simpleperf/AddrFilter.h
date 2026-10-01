/*
 * Copyright (C) 2025 The Android Open Source Project
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

#pragma once

#include <inttypes.h>

#include <string>
#include <vector>

namespace simpleperf {

struct AddrFilter {
  enum Type {
    FILE_RANGE,
    FILE_START,
    FILE_STOP,
    KERNEL_RANGE,
    KERNEL_START,
    KERNEL_STOP,
  } type;
  uint64_t addr;
  uint64_t size;
  std::string file_path;

  AddrFilter(AddrFilter::Type type, uint64_t addr, uint64_t size, const std::string& file_path)
      : type(type), addr(addr), size(size), file_path(file_path) {}

  std::string ToString() const;
};

std::vector<AddrFilter> ParseAddrFilterOption(const std::string& s);

}  // namespace simpleperf
