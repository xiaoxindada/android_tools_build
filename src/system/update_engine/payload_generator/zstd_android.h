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

#ifndef UPDATE_ENGINE_PAYLOAD_GENERATOR_ZSTD_ANDROID_H_
#define UPDATE_ENGINE_PAYLOAD_GENERATOR_ZSTD_ANDROID_H_

#include <brillo/brillo_export.h>
#include <brillo/secure_blob.h>

namespace chromeos_update_engine {

// Compresses the data in |in| and stores it in |out|. Returns true on
// success, false otherwise.
BRILLO_EXPORT bool ZstdCompress(const brillo::Blob& in, brillo::Blob* out);

// Decompresses the data in |in| and stores it in |out|. Returns true on
// success, false otherwise.
BRILLO_EXPORT bool ZstdDecompress(const brillo::Blob& in, brillo::Blob* out);

}  // namespace chromeos_update_engine

#endif  // UPDATE_ENGINE_PAYLOAD_GENERATOR_ZSTD_ANDROID_H_
