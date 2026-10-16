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

#include <gtest/gtest.h>

#include <android-base/file.h>
#include <android-base/test_utils.h>
#include <filesystem>

#include "AddrFilter.h"
#include "get_test_data.h"

using namespace simpleperf;
namespace fs = std::filesystem;

// @CddTest = 6.1/C-0-2
class AddrFilterOptionTest : public testing::Test {
 protected:
  std::string ParseOption(const std::string& option) {
    auto filters = ParseAddrFilterOption(option);
    std::string s;
    for (auto& filter : filters) {
      if (!s.empty()) {
        s += ',';
      }
      s += filter.ToString();
    }
    return s;
  };
};

// @CddTest = 6.1/C-0-2
TEST_F(AddrFilterOptionTest, filter_on_elf_file) {
  std::string path;
  ASSERT_TRUE(android::base::Realpath(GetTestData(ELF_FILE), &path));

  // Test file filters.
  ASSERT_EQ(ParseOption("filter " + path), "filter 0x0/0x21b1@" + path);
  ASSERT_EQ(ParseOption("filter 0x400502-0x400527@" + path), "filter 0x502/0x25@" + path);
  ASSERT_EQ(ParseOption("start 0x400502@" + path + ",stop 0x400527@" + path),
            "start 0x502@" + path + ",stop 0x527@" + path);

  // Test '-' in file path. Create a temporary file with '-' in name.
  TemporaryDir tmpdir;
  fs::path tmpfile = fs::path(tmpdir.path) / "elf-with-hyphen";
  ASSERT_TRUE(fs::copy_file(path, tmpfile));
  ASSERT_EQ(ParseOption("filter " + tmpfile.string()), "filter 0x0/0x21b1@" + tmpfile.string());
}

// @CddTest = 6.1/C-0-2
TEST_F(AddrFilterOptionTest, filter_on_apk_file) {
  // Android app may use native libraries embedded in the apk file.
  // Testing using the whole apk file as the addr filter.
  std::string path;
  ASSERT_TRUE(android::base::Realpath(GetTestData("EndlessTunnel.apk"), &path));
  ASSERT_EQ(ParseOption("filter " + path), "filter 0x0/0x2511c4@" + path);
  ASSERT_EQ(ParseOption("filter 0x120000-0x18c6d0@" + path), "filter 0x120000/0x6c6d0@" + path);
  ASSERT_EQ(ParseOption("start 0x120000@" + path + ",stop 0x18c6d0@" + path),
            "start 0x120000@" + path + ",stop 0x18c6d0@" + path);
}

// @CddTest = 6.1/C-0-2
TEST_F(AddrFilterOptionTest, filter_on_binary_embedded_in_apk_file) {
  // Testing using a native binary embedded in the apk file.
  std::string path;
  ASSERT_TRUE(android::base::Realpath(GetTestData("EndlessTunnel.apk"), &path));
  std::string embedded_path = path + "!/lib/arm64-v8a/libgame.so";
  ASSERT_EQ(ParseOption("filter " + embedded_path), "filter 0x120000/0x6c6d0@" + path);
  ASSERT_EQ(ParseOption("filter 0x1b320-0x4cc58@" + embedded_path),
            "filter 0x13b320/0x31938@" + path);
  ASSERT_EQ(ParseOption("start 0x1b320@" + embedded_path + ",stop 0x4cc58@" + embedded_path),
            "start 0x13b320@" + path + ",stop 0x16cc58@" + path);
}

// @CddTest = 6.1/C-0-2
TEST_F(AddrFilterOptionTest, filter_on_kernel_address) {
  // Test kernel filters.
  ASSERT_EQ(ParseOption("filter 0x12345678-0x1234567a"), "filter 0x12345678/0x2");
  ASSERT_EQ(ParseOption("start 0x12345678,stop 0x1234567a"), "start 0x12345678,stop 0x1234567a");
}
