// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "bsdiff/decompressor_interface.h"

#include "bsdiff/brotli_decompressor.h"
#include "bsdiff/bz2_decompressor.h"
#include "bsdiff/logging.h"

namespace bsdiff {

std::unique_ptr<DecompressorInterface> CreateDecompressor(
    CompressorType type,
    const uint8_t* input_data,
    size_t size) {
  switch (type) {
    case CompressorType::kBZ2: {
      auto decompressor = std::make_unique<BZ2MemoryDecompressor>();
      if (decompressor->SetInputData(input_data, size))
        return decompressor;
      return nullptr;
    }
    case CompressorType::kBrotli: {
      auto decompressor = std::make_unique<BrotliMemoryDecompressor>();
      if (decompressor->SetInputData(input_data, size))
        return decompressor;
      return nullptr;
    }
    default:
      LOG(ERROR) << "unsupported compressor type: "
                 << static_cast<uint8_t>(type);
      return nullptr;
  }
}

std::unique_ptr<DecompressorInterface> CreateDecompressor(CompressorType type,
                                                          int fd,
                                                          off_t offset,
                                                          size_t size) {
  switch (type) {
    case CompressorType::kBZ2: {
      auto decompressor = std::make_unique<BZ2FileDecompressor>();
      if (decompressor->SetInputFile(fd, offset, size)) {
        return decompressor;
      }
      return nullptr;
    }
    case CompressorType::kBrotli: {
      auto decompressor = std::make_unique<BrotliFileDecompressor>();
      if (decompressor->SetInputFile(fd, offset, size)) {
        return decompressor;
      }
      return nullptr;
    }
    default:
      LOG(ERROR) << "unsupported compressor type: "
                 << static_cast<uint8_t>(type);
      return nullptr;
  }
}

}  // namespace bsdiff
