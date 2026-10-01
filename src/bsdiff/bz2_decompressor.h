// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef _BSDIFF_BZ2_DECOMPRESSOR_H_
#define _BSDIFF_BZ2_DECOMPRESSOR_H_

#include <bzlib.h>
#include <string_view>
#include <vector>

#include "bsdiff/decompressor_interface.h"

namespace bsdiff {

class BZ2DecompressorCommon : public DecompressorInterface {
 public:
  BZ2DecompressorCommon() = default;
  ~BZ2DecompressorCommon() override;

  bool Read(uint8_t* output_data, size_t bytes_to_output) override;
  bool Close() override;

 protected:
  [[nodiscard]] virtual std::string_view GetNextInputChunk() = 0;
  [[nodiscard]] virtual size_t RemainingInputSize() const = 0;

 private:
  bz_stream stream_;
  bool stream_initialized_{false};
};

class BZ2MemoryDecompressor final : public BZ2DecompressorCommon {
 public:
  BZ2MemoryDecompressor() = default;
  ~BZ2MemoryDecompressor() override = default;

  bool SetInputData(const uint8_t* input_data, size_t size);

 protected:
  std::string_view GetNextInputChunk() override;
  size_t RemainingInputSize() const override;

 private:
  const uint8_t* input_data_{nullptr};
  size_t input_data_size_{0};
};

class BZ2FileDecompressor : public BZ2DecompressorCommon {
 public:
  BZ2FileDecompressor() = default;
  ~BZ2FileDecompressor() override;

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

#endif
