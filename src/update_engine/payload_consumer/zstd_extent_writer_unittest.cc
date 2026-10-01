//
// Copyright (C) 2025 The Android Open Source Project
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

#include "update_engine/payload_consumer/zstd_extent_writer.h"

#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#include <algorithm>
#include <memory>

#include <base/memory/ptr_util.h>
#include <gtest/gtest.h>

#include "update_engine/common/test_utils.h"
#include "update_engine/common/utils.h"
#include "update_engine/payload_consumer/fake_extent_writer.h"

namespace chromeos_update_engine {

namespace {

const char kSampleData[] = "Redundaaaaaaaaaaaaaant\n";

// Compressed data, generated with:
// echo "Redundaaaaaaaaaaaaaant" | zstd |
// hexdump -v -e '"    " 12/1 "0x%02x, " "\n"'
const uint8_t kCompressedData[] = {
    0x28, 0xb5, 0x2f, 0xfd, 0x04, 0x58, 0x85, 0x00, 0x00, 0x50,
    0x52, 0x65, 0x64, 0x75, 0x6e, 0x64, 0x61, 0x6e, 0x74, 0x0a,
    0x01, 0x00, 0x07, 0x30, 0x02, 0xeb, 0x10, 0x71, 0x5f,
};

// Highly redundant data bigger than the internal buffer, generated with:
// dd if=/dev/zero bs=30K count=1 | tr '\0' 'a' | zstd |
// hexdump -v -e '"    " 12/1 "0x%02x, " "\n"'
const uint8_t kCompressed30KiBofA[] = {
    0x28, 0xb5, 0x2f, 0xfd, 0x04, 0x58, 0x4d, 0x00, 0x00, 0x10, 0x61,
    0x61, 0x01, 0x00, 0xfb, 0xf7, 0x0e, 0xb0, 0x8c, 0x53, 0xb8, 0x48,
};

}  // namespace

class ZstdExtentWriterTest : public ::testing::Test {
 protected:
  void SetUp() override {
    fake_extent_writer_ = new FakeExtentWriter();
    zstd_writer_.reset(
        new ZstdExtentWriter(base::WrapUnique(fake_extent_writer_)));
  }

  void WriteAll(const brillo::Blob& compressed) {
    ASSERT_TRUE(zstd_writer_->Init({}, 1024));
    ASSERT_TRUE(zstd_writer_->Write(compressed.data(), compressed.size()));

    ASSERT_TRUE(fake_extent_writer_->InitCalled());
  }

  // Owned by |zstd_writer_|. This object is invalidated after |zstd_writer_|
  // is deleted.
  FakeExtentWriter* fake_extent_writer_{nullptr};
  std::unique_ptr<ZstdExtentWriter> zstd_writer_;

  const brillo::Blob sample_data_{
      std::begin(kSampleData), std::begin(kSampleData) + strlen(kSampleData)};
  FileDescriptorPtr fd_;
};

TEST_F(ZstdExtentWriterTest, CreateAndDestroy) {
  // Test that no Init() or End() called doesn't crash the program.
  ASSERT_FALSE(fake_extent_writer_->InitCalled());
}

TEST_F(ZstdExtentWriterTest, CompressedSampleData) {
  ASSERT_NO_FATAL_FAILURE(WriteAll(
      brillo::Blob(std::begin(kCompressedData), std::end(kCompressedData))));
  ASSERT_EQ(sample_data_, fake_extent_writer_->WrittenData());
}

TEST_F(ZstdExtentWriterTest, CompressedDataBiggerThanTheBuffer) {
  // Test that even if the output data is bigger than the internal buffer, all
  // the data is written.
  ASSERT_NO_FATAL_FAILURE(WriteAll(brillo::Blob(
      std::begin(kCompressed30KiBofA), std::end(kCompressed30KiBofA))));
  brillo::Blob expected_data(30 * 1024, 'a');
  ASSERT_EQ(expected_data, fake_extent_writer_->WrittenData());
}

TEST_F(ZstdExtentWriterTest, GarbageDataRejected) {
  ASSERT_TRUE(zstd_writer_->Init({}, 1024));
  // The sample_data_ is an uncompressed string.
  ASSERT_FALSE(zstd_writer_->Write(sample_data_.data(), sample_data_.size()));
}

TEST_F(ZstdExtentWriterTest, PartialDataIsKept) {
  brillo::Blob compressed(std::begin(kCompressed30KiBofA),
                          std::end(kCompressed30KiBofA));
  ASSERT_TRUE(zstd_writer_->Init({}, 1024));
  for (uint8_t byte : compressed) {
    ASSERT_TRUE(zstd_writer_->Write(&byte, 1));
  }

  // The sample_data_ is an uncompressed string.
  brillo::Blob expected_data(30 * 1024, 'a');
  ASSERT_EQ(expected_data, fake_extent_writer_->WrittenData());
}

}  // namespace chromeos_update_engine
