// Copyright (C) 2018 The Android Open Source Project
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

#include <unordered_set>

#include <android-base/file.h>
#include <gtest/gtest.h>
#include <libsnapshot/cow_writer.h>
#include <payload_consumer/file_descriptor.h>

namespace android {
namespace snapshot {

using android::base::unique_fd;
using chromeos_update_engine::FileDescriptor;

static constexpr uint32_t kBlockSize = 4096;
static constexpr size_t kBlockCount = 10;

class OfflineSnapshotTest : public ::testing::Test {
  protected:
    virtual void SetUp() override {
        base_ = std::make_unique<TemporaryFile>();
        ASSERT_GE(base_->fd, 0) << strerror(errno);

        cow_ = std::make_unique<TemporaryFile>();
        ASSERT_GE(cow_->fd, 0) << strerror(errno);

        WriteBaseDevice();
    }

    virtual void TearDown() override {
        base_ = nullptr;
        cow_ = nullptr;
        base_blocks_ = {};
    }

    void WriteBaseDevice() {
        unique_fd random(open("/dev/urandom", O_RDONLY));
        ASSERT_GE(random, 0);

        for (size_t i = 0; i < kBlockCount; i++) {
            std::string block(kBlockSize, 0);
            ASSERT_TRUE(android::base::ReadFully(random, block.data(), block.size()));
            ASSERT_TRUE(android::base::WriteFully(base_->fd, block.data(), block.size()));
            base_blocks_.emplace_back(std::move(block));
        }
        ASSERT_EQ(fsync(base_->fd), 0);
    }

    void WriteCow(ICowWriter* writer) {
        std::string new_block = MakeNewBlockString();
        std::string xor_block = MakeXorBlockString();

        ASSERT_TRUE(writer->AddXorBlocks(1, xor_block.data(), xor_block.size(), 0, kBlockSize / 2));
        ASSERT_TRUE(writer->AddCopy(3, 0));
        ASSERT_TRUE(writer->AddRawBlocks(5, new_block.data(), new_block.size()));
        ASSERT_TRUE(writer->AddZeroBlocks(7, 2));
        ASSERT_TRUE(writer->Finalize());
    }

    void TestBlockReads(ICowWriter* writer) {
        auto reader = writer->OpenFileDescriptor(base_->path);
        ASSERT_NE(reader, nullptr);

        // Test that unchanged blocks are not modified.
        std::unordered_set<size_t> changed_blocks = {1, 3, 5, 7, 8};
        for (size_t i = 0; i < kBlockCount; i++) {
            if (changed_blocks.count(i)) {
                continue;
            }

            std::string block(kBlockSize, 0);
            ASSERT_EQ(reader->Seek(i * kBlockSize, SEEK_SET), i * kBlockSize);
            ASSERT_EQ(reader->Read(block.data(), block.size()), kBlockSize);
            ASSERT_EQ(block, base_blocks_[i]);
        }

        // Test that we can read back our modified blocks.
        std::string data(kBlockSize, 0);
        std::string offsetblock = base_blocks_[0].substr(kBlockSize / 2, kBlockSize / 2) +
                                  base_blocks_[1].substr(0, kBlockSize / 2);
        ASSERT_EQ(offsetblock.size(), kBlockSize);
        ASSERT_EQ(reader->Seek(1 * kBlockSize, SEEK_SET), 1 * kBlockSize);
        ASSERT_EQ(reader->Read(data.data(), data.size()), kBlockSize);
        for (int i = 0; i < 100; i++) {
            data[i] = (char)~(data[i]);
        }
        ASSERT_EQ(data, offsetblock);

        std::string block(kBlockSize, 0);
        ASSERT_EQ(reader->Seek(3 * kBlockSize, SEEK_SET), 3 * kBlockSize);
        ASSERT_EQ(reader->Read(block.data(), block.size()), kBlockSize);
        ASSERT_EQ(block, base_blocks_[0]);

        ASSERT_EQ(reader->Seek(5 * kBlockSize, SEEK_SET), 5 * kBlockSize);
        ASSERT_EQ(reader->Read(block.data(), block.size()), kBlockSize);
        ASSERT_EQ(block, MakeNewBlockString());

        std::string two_blocks(kBlockSize * 2, 0x7f);
        std::string zeroes(kBlockSize * 2, 0);
        ASSERT_EQ(reader->Seek(7 * kBlockSize, SEEK_SET), 7 * kBlockSize);
        ASSERT_EQ(reader->Read(two_blocks.data(), two_blocks.size()), two_blocks.size());
        ASSERT_EQ(two_blocks, zeroes);
    }

    void TestByteReads(ICowWriter* writer) {
        auto reader = writer->OpenFileDescriptor(base_->path);
        ASSERT_NE(reader, nullptr);

        std::string blob(kBlockSize * 3, 'x');

        // Test that we can read in the middle of a block.
        static constexpr size_t kOffset = 970;
        off64_t offset = 3 * kBlockSize + kOffset;
        ASSERT_EQ(reader->Seek(0, SEEK_SET), 0);
        ASSERT_EQ(reader->Seek(offset, SEEK_CUR), offset);
        ASSERT_EQ(reader->Read(blob.data(), blob.size()), blob.size());
        ASSERT_EQ(blob.substr(0, 100), base_blocks_[0].substr(kOffset, 100));
        ASSERT_EQ(blob.substr(kBlockSize - kOffset, kBlockSize), base_blocks_[4]);
        ASSERT_EQ(blob.substr(kBlockSize * 2 - kOffset, 100), MakeNewBlockString().substr(0, 100));
        ASSERT_EQ(blob.substr(blob.size() - kOffset), base_blocks_[6].substr(0, kOffset));

        // Pull a random byte from the compressed block.
        char value;
        offset = 5 * kBlockSize + 1000;
        ASSERT_EQ(reader->Seek(offset, SEEK_SET), offset);
        ASSERT_EQ(reader->Read(&value, sizeof(value)), sizeof(value));
        ASSERT_EQ(value, MakeNewBlockString()[1000]);

        // Test a sequence of one byte reads.
        offset = 5 * kBlockSize + 10;
        std::string expected = MakeNewBlockString().substr(10, 20);
        ASSERT_EQ(reader->Seek(offset, SEEK_SET), offset);

        std::string got;
        while (got.size() < expected.size()) {
            ASSERT_EQ(reader->Read(&value, sizeof(value)), sizeof(value));
            got.push_back(value);
        }
        ASSERT_EQ(got, expected);
    }

    void TestReads(ICowWriter* writer) {
        ASSERT_NO_FATAL_FAILURE(TestBlockReads(writer));
        ASSERT_NO_FATAL_FAILURE(TestByteReads(writer));
    }

    std::string MakeNewBlockString() {
        std::string new_block = "This is a new block";
        new_block.resize(kBlockSize / 2, '*');
        new_block.resize(kBlockSize, '!');
        return new_block;
    }

    std::string MakeXorBlockString() {
        std::string data(kBlockSize, 0);
        memset(data.data(), 0xff, 100);
        return data;
    }

    std::unique_ptr<TemporaryFile> base_;
    std::unique_ptr<TemporaryFile> cow_;
    std::vector<std::string> base_blocks_;
};

TEST_F(OfflineSnapshotTest, CompressedSnapshot) {
    CowOptions options;
    options.compression = "gz";
    options.max_blocks = {kBlockCount};
    options.scratch_space = false;

    unique_fd cow_fd(dup(cow_->fd));
    ASSERT_GE(cow_fd, 0);

    auto writer = CreateCowWriter(2, options, std::move(cow_fd));
    ASSERT_NO_FATAL_FAILURE(WriteCow(writer.get()));
    ASSERT_NO_FATAL_FAILURE(TestReads(writer.get()));
}

TEST_F(OfflineSnapshotTest, ExtentMapMergeLeftAndRight) {
    CowOptions options;
    options.compression = "gz";
    options.max_blocks = {kBlockCount};
    options.scratch_space = false;

    unique_fd cow_fd(dup(cow_->fd));
    ASSERT_GE(cow_fd, 0);

    auto writer = CreateCowWriter(2, options, std::move(cow_fd));
    ASSERT_NE(writer, nullptr);

    // 1. Merge with left and right:
    // Insert 0, then 2, then 1 -> Should merge into a single Extent(0, 3)
    ASSERT_TRUE(writer->AddZeroBlocks(0, 1));
    ASSERT_TRUE(writer->AddZeroBlocks(2, 1));
    ASSERT_TRUE(writer->AddZeroBlocks(1, 1));

    ASSERT_TRUE(writer->Finalize());

    auto reader = writer->OpenFileDescriptor(base_->path);
    ASSERT_NE(reader, nullptr);

    // Read to verify ExtentMap merges and spans correctly

    // Verify 1: Zero blocks merged (0, 3)
    std::string buffer(kBlockSize * 3, 0x7f);
    std::string zeroes(kBlockSize * 3, 0);
    ASSERT_EQ(reader->Seek(0, SEEK_SET), 0);
    ASSERT_EQ(reader->Read(buffer.data(), buffer.size()), buffer.size());
    ASSERT_EQ(buffer, zeroes);
}

TEST_F(OfflineSnapshotTest, ExtentMapMergeDiscontiguousToContiguous) {
    CowOptions options;
    options.compression = "gz";
    options.max_blocks = {kBlockCount};
    options.scratch_space = false;

    unique_fd cow_fd(dup(cow_->fd));
    ASSERT_GE(cow_fd, 0);

    auto writer = CreateCowWriter(2, options, std::move(cow_fd));
    ASSERT_NE(writer, nullptr);

    // 2. Discontiguous to contiguous:
    // Insert CopyOp 5 (source 8), then CopyOp 4 (source 7)
    // They are logically contiguous: 4 -> 7, 5 -> 8
    // Should merge into Extent(4, 2)
    ASSERT_TRUE(writer->AddCopy(5, 8));
    ASSERT_TRUE(writer->AddCopy(4, 7));

    ASSERT_TRUE(writer->Finalize());

    auto reader = writer->OpenFileDescriptor(base_->path);
    ASSERT_NE(reader, nullptr);

    // Verify 2: Copy blocks merged (4, 2)
    std::string copy_blocks = base_blocks_[7] + base_blocks_[8];
    std::string buffer(kBlockSize * 2, 0);
    ASSERT_EQ(reader->Seek(4 * kBlockSize, SEEK_SET), 4 * kBlockSize);
    ASSERT_EQ(reader->Read(buffer.data(), buffer.size()), buffer.size());
    ASSERT_EQ(buffer, copy_blocks);

    // Read partially across merged copy blocks
    std::string partially(kBlockSize, 0);
    ASSERT_EQ(reader->Seek(4 * kBlockSize + kBlockSize / 2, SEEK_SET),
              4 * kBlockSize + kBlockSize / 2);
    ASSERT_EQ(reader->Read(partially.data(), partially.size()), partially.size());
    std::string expected_partially = base_blocks_[7].substr(kBlockSize / 2, kBlockSize / 2) +
                                     base_blocks_[8].substr(0, kBlockSize / 2);
    ASSERT_EQ(partially, expected_partially);
}

TEST_F(OfflineSnapshotTest, ExtentMapReadAcrossDifferentExtentTypes) {
    CowOptions options;
    options.compression = "gz";
    options.max_blocks = {kBlockCount};
    options.scratch_space = false;

    unique_fd cow_fd(dup(cow_->fd));
    ASSERT_GE(cow_fd, 0);

    auto writer = CreateCowWriter(2, options, std::move(cow_fd));
    ASSERT_NE(writer, nullptr);

    // Setup for spanning read:
    // Copy blocks 4, 5
    ASSERT_TRUE(writer->AddCopy(5, 8));
    ASSERT_TRUE(writer->AddCopy(4, 7));

    // 3. Separate non-mergeable ops:
    std::string new_block = MakeNewBlockString();
    ASSERT_TRUE(writer->AddRawBlocks(6, new_block.data(), kBlockSize));
    ASSERT_TRUE(writer->AddRawBlocks(7, new_block.data(), kBlockSize));

    ASSERT_TRUE(writer->Finalize());

    auto reader = writer->OpenFileDescriptor(base_->path);
    ASSERT_NE(reader, nullptr);

    // Verify 3: Raw blocks (6, 2) and spanning across different extents types
    std::string spanning_buffer(kBlockSize * 4, 0x7f);
    // Span blocks 4, 5 (Copy), 6, 7 (Raw)
    std::string spanning_expected = base_blocks_[7] + base_blocks_[8] + new_block + new_block;
    ASSERT_EQ(reader->Seek(4 * kBlockSize, SEEK_SET), 4 * kBlockSize);
    ASSERT_EQ(reader->Read(spanning_buffer.data(), spanning_buffer.size()), spanning_buffer.size());
    ASSERT_EQ(spanning_buffer, spanning_expected);
}

TEST_F(OfflineSnapshotTest, ExtentMapNoMergeCopyOps) {
    CowOptions options;
    options.compression = "gz";
    options.max_blocks = {kBlockCount};
    options.scratch_space = false;

    unique_fd cow_fd(dup(cow_->fd));
    ASSERT_GE(cow_fd, 0);

    auto writer = CreateCowWriter(2, options, std::move(cow_fd));
    ASSERT_NE(writer, nullptr);

    ASSERT_TRUE(writer->AddCopy(2, 0));
    ASSERT_TRUE(writer->AddCopy(3, 5));

    ASSERT_TRUE(writer->Finalize());

    auto reader = writer->OpenFileDescriptor(base_->path);
    ASSERT_NE(reader, nullptr);

    // Read blocks 2,3 make sure they match base block 0,5
    std::string buffer(kBlockSize * 2, 0);
    ASSERT_EQ(reader->Seek(2 * kBlockSize, SEEK_SET), 2 * kBlockSize);
    ASSERT_EQ(reader->Read(buffer.data(), buffer.size()), buffer.size());

    std::string expected = base_blocks_[0] + base_blocks_[5];
    ASSERT_EQ(buffer, expected);
}

}  // namespace snapshot
}  // namespace android
