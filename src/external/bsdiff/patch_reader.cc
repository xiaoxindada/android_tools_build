// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "bsdiff/patch_reader.h"

#include <string.h>

#include <unistd.h>
#include <vector>

#include <bsdiff/decompressor_interface.h>
#include "bsdiff/constants.h"
#include "bsdiff/logging.h"
#include "bsdiff/utils.h"

namespace bsdiff {

bool BsdiffPatchReader::ParseHeader(
    const uint8_t* header,
    size_t patch_size,
    std::function<
        std::unique_ptr<DecompressorInterface>(CompressorType, size_t, size_t)>
        create_decompressor) {
  //   File format:
  //   0       8    magic header
  //   8       8    X
  //   16      8    Y
  //   24      8    new_file_size
  //   32      X    compressed control block
  //   32+X    Y    compressed diff block
  //   32+X+Y  ???  compressed extra block
  // with control block a set of triples (x,y,z) meaning "add x bytes
  // from oldfile to x bytes from the diff block; copy y bytes from the
  // extra block; seek forwards in oldfile by z bytes".

  if (patch_size < 32) {
    LOG(ERROR) << "Too small to be a bspatch. " << patch_size;
    return false;
  }
  // Check for appropriate magic.
  std::vector<CompressorType> compression_type;
  if (memcmp(header, kLegacyMagicHeader, 8) == 0) {
    // The magic header is "BSDIFF40" for legacy format.
    compression_type = {CompressorType::kBZ2, CompressorType::kBZ2,
                        CompressorType::kBZ2};
  } else if (memcmp(header, kBSDF2MagicHeader, 5) == 0) {
    // The magic header for BSDF2 format:
    // 0 5 BSDF2
    // 5 1 compressed type for control stream
    // 6 1 compressed type for diff stream
    // 7 1 compressed type for extra stream
    for (size_t i = 5; i < 8; i++) {
      uint8_t type = header[i];
      switch (type) {
        case static_cast<uint8_t>(CompressorType::kBZ2):
          compression_type.push_back(CompressorType::kBZ2);
          break;
        case static_cast<uint8_t>(CompressorType::kBrotli):
          compression_type.push_back(CompressorType::kBrotli);
          break;
        default:
          LOG(ERROR) << "Unsupported compression type: "
                     << static_cast<int>(type);
          return false;
      }
    }
  } else {
    LOG(ERROR) << "Not a bsdiff patch.";
    return false;
  }

  // Read lengths from header.
  int64_t ctrl_len = ParseInt64(header + 8);
  int64_t diff_len = ParseInt64(header + 16);
  int64_t signed_newsize = ParseInt64(header + 24);
  // We already checked that the patch_size is at least 32 bytes.
  if ((ctrl_len < 0) || (diff_len < 0) || (signed_newsize < 0) ||
      (static_cast<int64_t>(patch_size) - 32 < ctrl_len) ||
      (static_cast<int64_t>(patch_size) - 32 - ctrl_len < diff_len)) {
    LOG(ERROR) << "Corrupt patch.  ctrl_len: " << ctrl_len
               << ", data_len: " << diff_len
               << ", new_file_size: " << signed_newsize
               << ", patch_size: " << patch_size;
    return false;
  }
  new_file_size_ = signed_newsize;

  size_t offset = 32;
  ctrl_stream_ = create_decompressor(compression_type[0], offset, ctrl_len);
  if (!ctrl_stream_) {
    LOG(ERROR) << "Failed to init ctrl stream, ctrl_len: " << ctrl_len;
    return false;
  }

  offset += ctrl_len;
  diff_stream_ = create_decompressor(compression_type[1], offset, diff_len);
  if (!diff_stream_) {
    LOG(ERROR) << "Failed to init ctrl stream, diff_len: " << diff_len;
    return false;
  }

  offset += diff_len;
  extra_stream_ =
      create_decompressor(compression_type[2], offset, patch_size - offset);
  if (!extra_stream_) {
    LOG(ERROR) << "Failed to init extra stream, extra_offset: " << offset
               << ", patch_size: " << patch_size;
    return false;
  }
  return true;
}

bool BsdiffPatchReader::Init(const uint8_t* patch_data, size_t patch_size) {
  if (patch_size < 32) {
    LOG(ERROR) << "Too small to be a bspatch. " << patch_size;
    return false;
  }
  return ParseHeader(
      patch_data, patch_size,
      [patch_data](CompressorType type, size_t offset, size_t size) {
        return CreateDecompressor(type, patch_data + offset, size);
      });
  return true;
}

bool BsdiffPatchReader::Init(int fd, off_t offset, size_t size) {
  if (size < 32) {
    LOG(ERROR) << "Too small to be a bspatch. " << size;
    return false;
  }
  uint8_t header[32];
  if (pread(fd, header, sizeof(header), offset) != sizeof(header)) {
    PLOG(ERROR) << "Failed to read patch header";
    return false;
  }
  return ParseHeader(
      header, size,
      [fd, offset](CompressorType type, size_t relative_offset,
                   size_t size) -> std::unique_ptr<DecompressorInterface> {
        return CreateDecompressor(type, fd, offset + relative_offset, size);
      });
}

bool BsdiffPatchReader::ParseControlEntry(ControlEntry* control_entry) {
  if (!control_entry)
    return false;

  uint8_t buf[8];
  if (!ctrl_stream_->Read(buf, 8))
    return false;
  int64_t diff_size = ParseInt64(buf);

  if (!ctrl_stream_->Read(buf, 8))
    return false;
  int64_t extra_size = ParseInt64(buf);

  // Sanity check.
  if (diff_size < 0 || extra_size < 0) {
    LOG(ERROR) << "Corrupt patch; diff_size: " << diff_size
               << ", extra_size: " << extra_size;
    return false;
  }

  control_entry->diff_size = diff_size;
  control_entry->extra_size = extra_size;

  if (!ctrl_stream_->Read(buf, 8))
    return false;
  control_entry->offset_increment = ParseInt64(buf);

  return true;
}

bool BsdiffPatchReader::ReadDiffStream(uint8_t* buf, size_t size) {
  return diff_stream_->Read(buf, size);
}

bool BsdiffPatchReader::ReadExtraStream(uint8_t* buf, size_t size) {
  return extra_stream_->Read(buf, size);
}

bool BsdiffPatchReader::Finish() {
  if (!ctrl_stream_->Close()) {
    LOG(ERROR) << "Failed to close the control stream";
    return false;
  }

  if (!diff_stream_->Close()) {
    LOG(ERROR) << "Failed to close the diff stream";
    return false;
  }

  if (!extra_stream_->Close()) {
    LOG(ERROR) << "Failed to close the extra stream";
    return false;
  }
  return true;
}

}  // namespace bsdiff
