// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "bsdiff/bz2_decompressor.h"

#include <algorithm>
#include <cstring>
#include <limits>

#include <bzlib.h>
#include <unistd.h>

#include "bsdiff/logging.h"

namespace bsdiff {

BZ2DecompressorCommon::~BZ2DecompressorCommon() {
  // Release the memory on destruction if needed.
  if (stream_initialized_)
    BZ2_bzDecompressEnd(&stream_);
}

bool BZ2DecompressorCommon::Read(uint8_t* output_data, size_t bytes_to_output) {
  if (!stream_initialized_) {
    memset(&stream_, 0, sizeof(stream_));
    int bz2err = BZ2_bzDecompressInit(&stream_, 0, 0);
    if (bz2err != BZ_OK) {
      LOG(ERROR) << "Failed to bzinit control stream: " << bz2err;
      return false;
    }
    stream_initialized_ = true;
  }

  stream_.next_out = reinterpret_cast<char*>(output_data);
  while (bytes_to_output > 0) {
    size_t chunk_size = std::min<size_t>(
        std::numeric_limits<unsigned int>::max(), bytes_to_output);
    stream_.avail_out = static_cast<unsigned int>(chunk_size);

    while (stream_.avail_out > 0) {
      if (stream_.avail_in == 0 && RemainingInputSize() > 0) {
        std::string_view chunk = GetNextInputChunk();
        if (chunk.empty()) {
          return false;
        }
        if (chunk.size() > std::numeric_limits<unsigned int>::max()) {
          LOG(ERROR) << "Input chunk too large for bz2";
          return false;
        }
        stream_.next_in = const_cast<char*>(chunk.data());
        stream_.avail_in = static_cast<unsigned int>(chunk.size());
      }

      int bz2err = BZ2_bzDecompress(&stream_);
      if (bz2err == BZ_STREAM_END) {
        if (stream_.avail_out > 0 || bytes_to_output > chunk_size) {
          LOG(ERROR) << "BZ2 stream ended but expected more bytes";
          return false;
        }
        return true;
      }
      if (bz2err != BZ_OK) {
        LOG(ERROR) << "Failed to decompress data, error: " << bz2err;
        return false;
      }
      if (stream_.avail_out > 0 && stream_.avail_in == 0 &&
          RemainingInputSize() == 0) {
        LOG(ERROR) << "Unexpected end of input";
        return false;
      }
    }
    bytes_to_output -= chunk_size;
  }
  return true;
}
bool BZ2DecompressorCommon::Close() {
  if (!stream_initialized_) {
    return true;
  }
  int bz2err = BZ2_bzDecompressEnd(&stream_);
  stream_initialized_ = false;
  if (bz2err != BZ_OK) {
    LOG(ERROR) << "BZ2_bzDecompressEnd returns with " << bz2err;
    return false;
  }
  return true;
}

bool BZ2MemoryDecompressor::SetInputData(const uint8_t* input_data,
                                         size_t size) {
  // TODO(xunchang) update the avail_in for size > 2GB.
  if (size > std::numeric_limits<unsigned int>::max()) {
    LOG(ERROR) << "Oversized input data" << size;
    return false;
  }
  input_data_ = input_data;
  input_data_size_ = size;
  return true;
}

std::string_view BZ2MemoryDecompressor::GetNextInputChunk() {
  std::string_view chunk(reinterpret_cast<const char*>(input_data_),
                         input_data_size_);
  input_data_ += input_data_size_;
  input_data_size_ = 0;
  return chunk;
}

size_t BZ2MemoryDecompressor::RemainingInputSize() const {
  return input_data_size_;
}


bool BZ2FileDecompressor::SetInputFile(int fd, off_t offset, size_t size) {
  fd_ = fd;
  offset_ = offset;
  remaining_input_size_ = size;
  input_buffer_.resize(std::min((size_t)1024 * 64, size));
  return true;
}


BZ2FileDecompressor::~BZ2FileDecompressor() {
  Close();
}

std::string_view BZ2FileDecompressor::GetNextInputChunk() {
  size_t to_read = std::min(input_buffer_.size(), remaining_input_size_);
  ssize_t rc = pread(fd_, input_buffer_.data(), to_read, offset_);
  if (rc < 0) {
    PLOG(ERROR) << "Failed to read from input file at offset " << offset_;
    return {};
  }
  if (rc == 0) {
    LOG(ERROR) << "Unexpected EOF reading from input file";
    return {};
  }
  offset_ += rc;
  remaining_input_size_ -= rc;
  return {reinterpret_cast<char*>(input_buffer_.data()),
          static_cast<size_t>(rc)};
}

size_t BZ2FileDecompressor::RemainingInputSize() const {
  return remaining_input_size_;
}

}  // namespace bsdiff
