//
// Copyright (C) 2020 The Android Open Source Project
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

#pragma once

#include <optional>

#include <android-base/file.h>
#include <libsnapshot/cow_reader.h>
#include <payload_consumer/file_descriptor.h>
#include "libsnapshot/cow_format.h"
#include "libsnapshot_cow/extent_map.h"

namespace android {
namespace snapshot {

struct CowOp final : public CowOperation {
    constexpr size_t num_blocks() const {
        if (type() == kCowReplaceOp) {
            return 1 << compression_bits();
        } else if (type() == kCowCopyOp || type() == kCowZeroOp) {
            // For COPY or ZERO ops, there's no data associated with this op, so
            // we use space of field |data_length| to store the number of ops;
            return std::max(data_length, 1u);
        } else {
            return 1;
        }
    }

    constexpr std::optional<CowOperation> ToDiskRepr(size_t block_idx) const {
        if (block_idx < new_block || block_idx >= new_block + num_blocks()) {
            return {};
        }
        CowOperation ret;
        ret.source_info_ = source_info_;
        if (ret.type() == kCowCopyOp) {
            ret.set_source(block_idx - new_block + source());
            ret.new_block = block_idx;
            ret.data_length = 0;
        } else if (ret.type() == kCowZeroOp) {
            ret.new_block = block_idx;
            ret.data_length = 0;
        } else {
            ret.new_block = new_block;
            ret.data_length = data_length;
        }
        return ret;
    }

    constexpr static std::optional<CowOp> Convert(const CowOperation& op) {
        CowOp ret;
        ret.new_block = op.new_block;
        ret.data_length = op.data_length;
        ret.source_info_ = op.source_info_;
        return {ret};
    }
};

struct CowOpMerge {
    constexpr std::optional<CowOp> Merge(const CowOp& a, const CowOp& b) const {
        if (a.type() != b.type()) {
            return {};
        }
        if (a.new_block > b.new_block) {
            return Merge(b, a);
        }
        if (a.new_block == b.new_block) {
            return {};
        }
        if (a.type() == CowOperationType::kCowCopyOp) {
            if (a.source() + a.num_blocks() != b.source()) {
                return {};
            }
            CowOp new_op;
            new_op.new_block = a.new_block;
            new_op.set_type(a.type());
            // For CopyOps, data_length is unused, so we piggy back this field
            // to store number of blocks
            new_op.data_length = a.num_blocks() + b.num_blocks();
            new_op.set_source(a.source());
            return {new_op};
        } else if (a.type() == CowOperationType::kCowZeroOp) {
            CowOp new_op;
            new_op.new_block = a.new_block;
            // For ZeroOps, data_length is unused, so we piggy back this field
            // to store number of blocks
            new_op.data_length = a.num_blocks() + b.num_blocks();
            new_op.set_type(a.type());
            return {new_op};
        }
        return {};
    }
};

class CompressedSnapshotReader : public chromeos_update_engine::FileDescriptor {
  public:
    CompressedSnapshotReader(std::unique_ptr<ICowReader>&& cow,
                             const std::optional<std::string>& source_device,
                             std::optional<uint64_t> block_dev_size);

    bool Open(const char* path, int flags, mode_t mode) override;
    bool Open(const char* path, int flags) override;
    ssize_t Write(const void* buf, size_t count) override;
    bool BlkIoctl(int request, uint64_t start, uint64_t length, int* result) override;
    ssize_t Read(void* buf, size_t count) override;
    off64_t Seek(off64_t offset, int whence) override;
    uint64_t BlockDevSize() override;
    bool Close() override;
    bool IsSettingErrno() override;
    bool IsOpen() override;
    bool Flush() override;

  private:
    ssize_t ReadBlock(uint64_t chunk, size_t start_offset, void* buffer, size_t size);
    android::base::borrowed_fd GetSourceFd();

    std::unique_ptr<ICowReader> cow_;
    uint32_t block_size_ = 0;

    std::optional<std::string> source_device_;
    android::base::unique_fd source_fd_;
    uint64_t block_device_size_ = 0;
    off64_t offset_ = 0;

    ExtentMap<Extent, CowOp, CowOpMerge> extent_map_;
    std::vector<const CowOperation*> ops_;
};

}  // namespace snapshot
}  // namespace android
