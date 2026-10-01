//
// Copyright (C) 2026 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#include <fcntl.h>
#include <unistd.h>

#include <string>
#include <vector>

#include <android-base/file.h>
#include <android-base/unique_fd.h>
#include "gtest/gtest.h"

#include "puffin/file_stream.h"
#include "puffin/memory_stream.h"
#include "puffin/src/include/puffin/common.h"
#include "puffin/src/include/puffin/puffdiff.h"
#include "puffin/src/include/puffin/puffpatch.h"
#include "puffin/src/include/puffin/utils.h"
#include "puffin/src/unittest_common.h"

namespace puffin {

namespace {

// Reads a file fully into a Buffer.
bool ReadFileToBuffer(const std::string& path, Buffer* out_buffer) {
  std::string content;
  if (!android::base::ReadFileToString(path, &content)) {
    return false;
  }
  out_buffer->assign(content.begin(), content.end());
  return true;
}

}  // namespace

class PuffpatchIntegrationTest : public testing::Test {
 protected:
  void SetUp() override {
    // Determine the paths to the generated test zips
    std::string exe_dir = android::base::GetExecutableDirectory();
    src_zip_path_ = exe_dir + "/src.zip";
    dst_zip_path_ = exe_dir + "/dst.zip";

    ASSERT_TRUE(ReadFileToBuffer(src_zip_path_, &src_zip_));
    ASSERT_TRUE(ReadFileToBuffer(dst_zip_path_, &dst_zip_));

    ASSERT_TRUE(LocateDeflatesInZipArchive(src_zip_, &src_deflates_));
    ASSERT_TRUE(LocateDeflatesInZipArchive(dst_zip_, &dst_deflates_));
  }

  std::string src_zip_path_;
  std::string dst_zip_path_;
  Buffer src_zip_;
  Buffer dst_zip_;
  std::vector<BitExtent> src_deflates_;
  std::vector<BitExtent> dst_deflates_;
};

TEST_F(PuffpatchIntegrationTest, MemoryAndFdPatchingMatchesTarget) {
  std::string patch_file_path;
  ASSERT_TRUE(MakeTempFile(&patch_file_path, nullptr));
  ScopedPathUnlinker patch_unlinker(patch_file_path);

  Buffer patch;
  // Generate diff
  ASSERT_TRUE(PuffDiff(MemoryStream::CreateForRead(src_zip_),
                       MemoryStream::CreateForRead(dst_zip_), src_deflates_,
                       dst_deflates_, {bsdiff::CompressorType::kBrotli},
                       PatchAlgorithm::kBsdiff, patch_file_path, &patch));

  // 1. Test memory-based PuffPatch
  Buffer patched_mem;
  auto mem_src_stream = MemoryStream::CreateForRead(src_zip_);
  auto mem_dst_stream = MemoryStream::CreateForWrite(&patched_mem);

  ASSERT_TRUE(PuffPatch(std::move(mem_src_stream), std::move(mem_dst_stream),
                        patch.data(), patch.size()));

  ASSERT_EQ(dst_zip_, patched_mem);

  // 2. Test File Descriptor based PuffPatch
  // PuffDiff requires a valid temporary file to write the intermediate bsdiff
  // patch, so we cannot pass an empty or null path to it. However, we can
  // reuse that same file now to write the final patch and use it for the FD
  // test, avoiding the creation of a second temporary file.
  android::base::unique_fd patch_fd(
      open(patch_file_path.c_str(), O_RDWR | O_TRUNC));
  ASSERT_GE(patch_fd.get(), 0);

  // Write patch data to the temporary file
  ASSERT_TRUE(
      android::base::WriteFully(patch_fd.get(), patch.data(), patch.size()));

  Buffer patched_fd;
  auto fd_src_stream = MemoryStream::CreateForRead(src_zip_);
  auto fd_dst_stream = MemoryStream::CreateForWrite(&patched_fd);

  // Apply patch from the FD
  ASSERT_TRUE(PuffPatch(std::move(fd_src_stream), std::move(fd_dst_stream),
                        patch_fd.get(),
                        0,  // patch_offset
                        patch.size()));

  ASSERT_EQ(dst_zip_, patched_fd);
}

}  // namespace puffin
