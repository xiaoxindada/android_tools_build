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

#include "AddrFilter.h"

#include <string.h>

#include <android-base/file.h>
#include <android-base/logging.h>
#include <android-base/strings.h>
#include <string>
#include <vector>

#include "read_apk.h"
#include "read_elf.h"
#include "utils.h"

namespace simpleperf {

namespace {

static bool ConsumeStr(const char*& p, const char* s) {
  if (strncmp(p, s, strlen(s)) == 0) {
    p += strlen(s);
    return true;
  }
  return false;
}

static bool ConsumeAddr(const char*& p, uint64_t* addr) {
  errno = 0;
  char* end;
  *addr = strtoull(p, &end, 0);
  if (errno == 0 && p != end) {
    p = end;
    return true;
  }
  return false;
}

// Convert addresses used in addr filter to file offsets, and do some check.
static bool ConvertFileAddrsToOffsets(std::string& path, uint64_t& begin_offset,
                                      uint64_t& end_offset, std::vector<uint64_t>& addrs) {
  ElfStatus status;

  auto tuple = SplitUrlInApk(path);
  if (!std::get<0>(tuple)) {
    std::string s;
    if (!android::base::Realpath(path, &s)) {
      return false;
    }
    path = s;
    uint64_t file_size = GetFileSize(path);
    begin_offset = 0;
    end_offset = file_size;
    // Case 1: It's a normal ELF file. Convert addrs to file offsets.
    ElfStatus status;
    if (auto elf = ElfFile::Open(path, &status); elf) {
      for (uint64_t& addr : addrs) {
        uint64_t off;
        if (!elf->VaddrToOff(addr, &off)) {
          return false;
        }
        addr = off;
      }
      return true;
    }
    // Case 2: It's an apk file. Check addrs <= file size.
    for (uint64_t addr : addrs) {
      if (addr > file_size) {
        return false;
      }
    }
    return true;
  }

  // Case 3: It's an ELF binary embedded in an apk file.
  EmbeddedElf* embedded_elf =
      ApkInspector::FindElfInApkByName(std::get<1>(tuple), std::get<2>(tuple));
  if (embedded_elf != nullptr) {
    begin_offset = embedded_elf->entry_offset();
    end_offset = begin_offset + embedded_elf->entry_size();
    if (auto elf = ElfFile::Open(path, &status); elf) {
      for (uint64_t& addr : addrs) {
        uint64_t off;
        if (!elf->VaddrToOff(addr, &off)) {
          return false;
        }
        addr = begin_offset + off;
      }
      if (!android::base::Realpath(std::get<1>(tuple), &path)) {
        return false;
      }
      return true;
    }
  }
  return false;
}

// To reduce function length, not all format errors are checked.
static bool ParseOneAddrFilter(const std::string& s, std::vector<AddrFilter>* filters) {
  std::vector<std::string> args = android::base::Split(s, " ");
  if (args.size() != 2) {
    return false;
  }

  std::string path;
  uint64_t begin_offset = 0;
  uint64_t end_offset = 0;
  std::vector<uint64_t> addrs(2, 0);

  if (auto p = s.data(); ConsumeStr(p, "start") && ConsumeAddr(p, &addrs[0])) {
    if (*p == '\0') {
      // start <kernel_addr>
      filters->emplace_back(AddrFilter::KERNEL_START, addrs[0], 0, "");
      return true;
    }
    if (ConsumeStr(p, "@") && *p != '\0') {
      // start <vaddr>@<binary_path> or <offset>@<apk_path>
      path = p;
      addrs.resize(1);
      if (ConvertFileAddrsToOffsets(path, begin_offset, end_offset, addrs)) {
        filters->emplace_back(AddrFilter::FILE_START, addrs[0], 0, path);
        return true;
      }
    }
  }
  if (auto p = s.data(); ConsumeStr(p, "stop") && ConsumeAddr(p, &addrs[0])) {
    if (*p == '\0') {
      // stop <kernel_addr>
      filters->emplace_back(AddrFilter::KERNEL_STOP, addrs[0], 0, "");
      return true;
    }
    if (ConsumeStr(p, "@") && *p != '\0') {
      // stop <vaddr>@<binary_path> or <offset>@<apk_path>
      path = p;
      addrs.resize(1);
      if (ConvertFileAddrsToOffsets(path, begin_offset, end_offset, addrs)) {
        filters->emplace_back(AddrFilter::FILE_STOP, addrs[0], 0, path);
        return true;
      }
    }
  }
  if (auto p = s.data(); ConsumeStr(p, "filter") && ConsumeAddr(p, &addrs[0]) &&
                         ConsumeStr(p, "-") && ConsumeAddr(p, &addrs[1]) && addrs[1] > addrs[0]) {
    if (*p == '\0') {
      // filter <kernel_addr_start>-<kernel_addr_end>
      filters->emplace_back(AddrFilter::KERNEL_RANGE, addrs[0], addrs[1] - addrs[0], "");
      return true;
    }
    if (ConsumeStr(p, "@") && *p != '\0') {
      // filter <vaddr_start>-<vaddr_end>@<binary_path> or <offset_start>-<offset_end>@<apk_path>
      path = p;
      if (ConvertFileAddrsToOffsets(path, begin_offset, end_offset, addrs)) {
        filters->emplace_back(AddrFilter::FILE_RANGE, addrs[0], addrs[1] - addrs[0], path);
        return true;
      }
    }
  }
  if (auto p = s.data(); ConsumeStr(p, "filter") && *p != '\0') {
    // filter <file_path>
    while (isspace(*p) && *p != '\0') {
      p++;
    }
    path = p;
    addrs.clear();
    if (ConvertFileAddrsToOffsets(path, begin_offset, end_offset, addrs)) {
      filters->emplace_back(AddrFilter::FILE_RANGE, begin_offset, end_offset - begin_offset, path);
      return true;
    }
  }
  return false;
}

}  // namespace

std::vector<AddrFilter> ParseAddrFilterOption(const std::string& s) {
  std::vector<AddrFilter> filters;
  for (const auto& str : android::base::Split(s, ",")) {
    if (!ParseOneAddrFilter(str, &filters)) {
      LOG(ERROR) << "failed to parse addr filter: " << str;
      return {};
    }
  }
  return filters;
}

}  // namespace simpleperf
