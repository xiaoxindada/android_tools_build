/*
 * Copyright (C) 2026 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <inttypes.h>

#include <map>
#include <memory>

#include <android-base/expected.h>

#include "event_type.h"
#include "perf_event.h"
#include "record.h"

namespace simpleperf {

class SPERecorder {
 public:
  static SPERecorder& GetInstance();
  int GetSPEEventType();
  uint64_t GetMinInterval();

  AuxTraceInfoRecord CreateAuxTraceInfoRecord();
  void ReadSpeMidrInfo(const std::vector<int>& cpus);
  std::string ParseSpeTypes(const std::string& name);
  bool FindSpeConfig(const std::string& name, uint64_t* ret_val);

 private:
  struct spe_midr_info_s {
    int cpu;            // CPU id
    bool spe_enabled;   // SPE enabled on this CPU
    uint64_t midr_val;  // CPU MIDR value
  };
  std::vector<struct spe_midr_info_s> spe_midr_info_;
  std::unordered_map<std::string, uint64_t> spe_config_;
  int event_type_ = 0;
};
}  // namespace simpleperf
