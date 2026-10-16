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

#include <zstd.h>

#include <utility>

#include <base/logging.h>

namespace chromeos_update_engine {

ZstdExtentWriter::ZstdExtentWriter(std::unique_ptr<ExtentWriter> next_writer)
    : next_writer_(std::move(next_writer)) {}

ZstdExtentWriter::~ZstdExtentWriter() {
  if (zstd_dstream_) {
    ZSTD_freeDStream(zstd_dstream_);
  }
}

bool ZstdExtentWriter::Init(
    const google::protobuf::RepeatedPtrField<Extent>& extents,
    uint32_t block_size) {
  if (!next_writer_->Init(extents, block_size)) {
    return false;
  }
  zstd_dstream_ = ZSTD_createDStream();
  if (!zstd_dstream_) {
    LOG(ERROR) << "ZSTD_createDStream() failed";
    return false;
  }
  size_t ret = ZSTD_initDStream(zstd_dstream_);
  if (ZSTD_isError(ret)) {
    LOG(ERROR) << "ZSTD_initDStream() failed: " << ZSTD_getErrorName(ret);
    return false;
  }
  out_buffer_.resize(ZSTD_DStreamOutSize());
  return true;
}

bool ZstdExtentWriter::Write(const void* bytes, size_t count) {
  ZSTD_inBuffer input = {bytes, count, 0};
  while (input.pos < input.size) {
    ZSTD_outBuffer output = {out_buffer_.data(), out_buffer_.size(), 0};
    size_t ret = ZSTD_decompressStream(zstd_dstream_, &output, &input);
    if (ZSTD_isError(ret)) {
      LOG(ERROR) << "ZSTD_decompressStream() failed: "
                 << ZSTD_getErrorName(ret);
      return false;
    }
    if (!next_writer_->Write(out_buffer_.data(), output.pos)) {
      return false;
    }
  }
  return true;
}

}  // namespace chromeos_update_engine
