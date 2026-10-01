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

#ifndef UPDATE_ENGINE_PAYLOAD_CONSUMER_ZSTD_EXTENT_WRITER_H_
#define UPDATE_ENGINE_PAYLOAD_CONSUMER_ZSTD_EXTENT_WRITER_H_

#include <zstd.h>

#include <memory>

#include <brillo/secure_blob.h>

#include "update_engine/payload_consumer/extent_writer.h"

namespace chromeos_update_engine {

class ZstdExtentWriter : public ExtentWriter {
 public:
  explicit ZstdExtentWriter(std::unique_ptr<ExtentWriter> next_writer);
  ~ZstdExtentWriter() override;

  // ExtentWriter methods.
  bool Init(const google::protobuf::RepeatedPtrField<Extent>& extents,
            uint32_t block_size) override;
  bool Write(const void* bytes, size_t count) override;

 private:
  std::unique_ptr<ExtentWriter> next_writer_;
  ZSTD_DStream* zstd_dstream_{nullptr};
  brillo::Blob out_buffer_;
};

}  // namespace chromeos_update_engine

#endif  // UPDATE_ENGINE_PAYLOAD_CONSUMER_ZSTD_EXTENT_WRITER_H_
