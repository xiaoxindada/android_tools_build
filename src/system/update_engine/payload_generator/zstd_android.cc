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

#include "update_engine/payload_generator/zstd_android.h"

#include <zstd.h>

#include <base/logging.h>

namespace chromeos_update_engine {

bool ZstdCompress(const brillo::Blob& in, brillo::Blob* out) {
  if (in.empty()) {
    out->clear();
    return true;
  }
  // Default compression level on zstd is ZSTD_defaultCLevel();
  // Let's set it to high so we get comparable compression wrt xz/bzip.
  // Decompression will still be very fast compared to xz (8-9x faster).
  int zstd_compress_level = 22;
  ZSTD_CCtx* const cctx = ZSTD_createCCtx();
  CHECK(cctx != nullptr);

  // Set the strategy to good enough, strongest and slowest being ZSTD_btultra2
  // which we chose not to use as it is very memory intensive.
  ZSTD_CCtx_setParameter(cctx, ZSTD_c_strategy, ZSTD_btopt);
  // Set the compression level to the default.
  ZSTD_CCtx_setParameter(cctx, ZSTD_c_compressionLevel, zstd_compress_level);
  size_t const bound = ZSTD_compressBound(in.size());
  out->resize(bound);

  size_t const compressed_size =
      ZSTD_compress2(cctx, out->data(), out->size(), in.data(), in.size());

  ZSTD_freeCCtx(cctx);

  if (ZSTD_isError(compressed_size)) {
    LOG(ERROR) << "ZSTD_compress2 failed: "
               << ZSTD_getErrorName(compressed_size);
    return false;
  }
  out->resize(compressed_size);
  return true;
}

bool ZstdDecompress(const brillo::Blob& in, brillo::Blob* out) {
  if (in.empty()) {
    out->clear();
    return true;
  }
  unsigned long long const decompressed_size =
      ZSTD_getFrameContentSize(in.data(), in.size());

  if (decompressed_size == ZSTD_CONTENTSIZE_ERROR ||
      decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN) {
    LOG(ERROR) << "ZSTD_getFrameContentSize failed";
    return false;
  }
  out->resize(decompressed_size);
  size_t const ret =
      ZSTD_decompress(out->data(), out->size(), in.data(), in.size());
  if (ZSTD_isError(ret) || ret != decompressed_size) {
    LOG(ERROR) << "ZSTD_decompress failed: " << ZSTD_getErrorName(ret);
    return false;
  }
  return true;
}

}  // namespace chromeos_update_engine
