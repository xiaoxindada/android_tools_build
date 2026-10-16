// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "bsdiff/brotli_decompressor.h"

#include <unistd.h>

#include "bsdiff/logging.h"


namespace bsdiff {


bool BrotliDecompressorCommon::Read(uint8_t* output_data,
                                    size_t bytes_to_output) {
  if (!brotli_decoder_state_) {
    brotli_decoder_state_ =
        BrotliDecoderCreateInstance(nullptr, nullptr, nullptr);
    if (brotli_decoder_state_ == nullptr) {
      LOG(ERROR) << "Failed to initialize brotli decoder.";
      return false;
    }
  }

  uint8_t* next_out = output_data;
  size_t available_out = bytes_to_output;

  while (available_out > 0) {
    if (available_in_ == 0 && RemainingInputSize() > 0) {
      std::string_view chunk = GetNextInputChunk();
      if (chunk.empty()) {
        return false;
      }
      next_in_ = reinterpret_cast<const uint8_t*>(chunk.data());
      available_in_ = chunk.size();
    }

    BrotliDecoderResult result = BrotliDecoderDecompressStream(
        brotli_decoder_state_, &available_in_, &next_in_, &available_out,
        &next_out, nullptr);

    if (result == BROTLI_DECODER_RESULT_ERROR) {
      LOG(ERROR) << "Decompression failed with "
                 << BrotliDecoderErrorString(
                        BrotliDecoderGetErrorCode(brotli_decoder_state_));
      return false;
    } else if (result == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT) {
      if (RemainingInputSize() == 0 && available_in_ == 0) {
        LOG(ERROR) << "Unexpected end of input, still need to decompress "
                   << available_out << " bytes";
        return false;
      }
    } else if (result == BROTLI_DECODER_RESULT_SUCCESS) {
      if (available_out > 0) {
        LOG(ERROR) << "Expected to read " << available_out
                   << " more bytes but reached the end of compressed brotli "
                      "stream";
        return false;
      }
      return true;
    }
  }
  return true;
}

BrotliDecompressorCommon::~BrotliDecompressorCommon() {
  Close();
}

bool BrotliDecompressorCommon::Close() {
  if (!brotli_decoder_state_) {
    return true;
  }
  // In some cases, the brotli compressed stream could be empty. As a result,
  // the function BrotliDecoderIsFinished() will return false because we never
  // start the decompression. When that happens, we just destroy the decoder
  // and return true.
  if (BrotliDecoderIsUsed(brotli_decoder_state_) &&
      !BrotliDecoderIsFinished(brotli_decoder_state_)) {
    LOG(ERROR) << "Unfinished brotli decoder.";
    return false;
  }

  BrotliDecoderDestroyInstance(brotli_decoder_state_);
  brotli_decoder_state_ = nullptr;
  return true;
}

bool BrotliMemoryDecompressor::SetInputData(const uint8_t* input_data,
                                            size_t size) {
  input_data_ =
      std::string_view(reinterpret_cast<const char*>(input_data), size);
  return true;
}

[[nodiscard]] std::string_view BrotliMemoryDecompressor::GetNextInputChunk() {
  auto ret = input_data_;
  input_data_ = {};
  return ret;
}

[[nodiscard]] size_t BrotliMemoryDecompressor::RemainingInputSize() const {
  return input_data_.size();
}


BrotliFileDecompressor::~BrotliFileDecompressor() = default;

bool BrotliFileDecompressor::SetInputFile(int fd, off_t offset, size_t size) {
  fd_ = fd;
  offset_ = offset;
  remaining_input_size_ = size;
  return true;
}

std::string_view BrotliFileDecompressor::GetNextInputChunk() {
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

size_t BrotliFileDecompressor::RemainingInputSize() const {
  return remaining_input_size_;
}

}  // namespace bsdiff
