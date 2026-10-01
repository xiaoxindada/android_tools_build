// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef _BSDIFF_BROTLI_DECOMPRESSOR_H_
#define _BSDIFF_BROTLI_DECOMPRESSOR_H_

#include <brotli/decode.h>
#include <sys/types.h>
#include <string_view>
#include <vector>

#include "bsdiff/decompressor_interface.h"

namespace bsdiff {

class BrotliDecompressorCommon : public DecompressorInterface {
 public:
  BrotliDecompressorCommon() = default;
  bool Read(uint8_t* output_data, size_t bytes_to_output);
  ~BrotliDecompressorCommon();
  bool Close() override;

 protected:
  [[nodiscard]] virtual std::string_view GetNextInputChunk() = 0;
  [[nodiscard]] virtual size_t RemainingInputSize() const = 0;

 private:
  size_t available_in_{};
  const uint8_t* next_in_;
  BrotliDecoderState* brotli_decoder_state_{};
};

class BrotliMemoryDecompressor final : public BrotliDecompressorCommon {
 public:
  // DecompressorInterface overrides.
  bool SetInputData(const uint8_t* input_data, size_t size);

 protected:
  [[nodiscard]] std::string_view GetNextInputChunk();
  [[nodiscard]] size_t RemainingInputSize() const;

 private:
  std::string_view input_data_;
};


class BrotliFileDecompressor : public BrotliDecompressorCommon {
 public:
  BrotliFileDecompressor() : fd_(-1), input_buffer_(64 * 1024) {}
  ~BrotliFileDecompressor() override;

  bool SetInputFile(int fd, off_t offset, size_t size);

 protected:
  [[nodiscard]] std::string_view GetNextInputChunk() override;
  [[nodiscard]] size_t RemainingInputSize() const override;

 private:
  int fd_;
  off_t offset_;
  size_t remaining_input_size_;
  std::vector<uint8_t> input_buffer_;
};

}  // namespace bsdiff

#endif  // _BSDIFF_BROTLI_DECOMPRESSOR_H_
