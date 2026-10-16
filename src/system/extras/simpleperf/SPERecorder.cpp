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

#include <stdio.h>

#include <string>

#include <android-base/expected.h>
#include <android-base/file.h>
#include <android-base/logging.h>
#include <android-base/parseint.h>
#include <android-base/properties.h>
#include <android-base/strings.h>

#include "SPERecorder.h"
#include "environment.h"
#include "utils.h"

namespace simpleperf {

static const std::string DEVICES_DIR = "/sys/bus/event_source/devices/";
static const uint64_t ARM_SPE_HEADER_VERSION = 1;

SPERecorder& SPERecorder::GetInstance() {
  static SPERecorder spe;
  return spe;
}

void SPERecorder::ReadSpeMidrInfo(const std::vector<int>& cpus) {
  std::vector<int> online_cpus = GetOnlineCpus();
  for (auto cpu : online_cpus) {
    uint64_t midr_val = 0;
    std::string s;
    bool spe_enabled = std::find(cpus.begin(), cpus.end(), cpu) != cpus.end();
    const std::string midr_path = android::base::StringPrintf(
        "/sys/devices/system/cpu/cpu%d/regs/identification/midr_el1", cpu);
    if (!android::base::ReadFileToString(midr_path, &s) ||
        !android::base::ParseUint(android::base::Trim(s), &midr_val)) {
      LOG(ERROR) << "SPE: CPU " << cpu << " MIDR path file read error; file path: " << midr_path;
      continue;
    }
    spe_midr_info_.push_back({cpu, spe_enabled, midr_val});
  }
}

std::string SPERecorder::ParseSpeTypes(const std::string& name) {
  size_t slash_pos = name.find('/');
  if (slash_pos == std::string::npos) {
    if (name == kSPEEventName) {
      return kSPEDefaultDeviceName;
    }
    return name;
  }
  std::string spe_device = name.substr(0, slash_pos);
  std::string config_string = name.substr(slash_pos + 1);
  size_t slash_pos2 = config_string.find('/');
  if ((slash_pos2 == std::string::npos) || (slash_pos2 != (config_string.size() - 1))) {
    LOG(ERROR) << "Invalid SPE input format, config parameters should be closed with a \"/\"";
    return name;
  }
  config_string.pop_back();  // remove '/' from the end
  std::stringstream config_stream(config_string);
  std::string config;
  while (std::getline(config_stream, config, ',')) {
    if (config.size() == 0) {
      continue;
    }
    size_t eq_pos = config.find('=');
    if (eq_pos == std::string::npos) {
      LOG(ERROR) << "Invalid config, missing '=<value>' : " << config;
      return name;
    }
    std::string config_name = config.substr(0, eq_pos);
    std::string value_as_str = config.substr(eq_pos + 1);
    uint64_t val = 0;
    if (android::base::ParseUint(android::base::Trim(value_as_str), &val)) {
      spe_config_.insert({config_name, val});
    } else {
      LOG(ERROR) << "Invalid config value, config: " << config << " value: " << value_as_str;
    }
  }
  if (spe_device == kSPEEventName) {
    spe_device = kSPEDefaultDeviceName;
  }
  return spe_device;
}

AuxTraceInfoRecord SPERecorder::CreateAuxTraceInfoRecord() {
  AuxTraceInfoRecord::DataType data;
  memset(&data, 0, sizeof(data));
  data.aux_type = AuxTraceInfoRecord::AUX_TYPE_SPE;
  data.version = ARM_SPE_HEADER_VERSION;
  data.nr_cpu = spe_midr_info_.size();
  data.pmu_type = GetSPEEventType();
  std::vector<AuxTraceInfoRecord::SPEInfo> spe(spe_midr_info_.size());
  size_t pos = 0;
  uint64_t cap_min_ival = GetMinInterval();
  for (auto& p : spe_midr_info_) {
    auto& e = spe[pos++];
    e.magic = AuxTraceInfoRecord::MAGIC_SPE;
    e.cpu = p.cpu;
    e.cpu_midr = p.midr_val;
    if (p.spe_enabled) {
      e.enabled = 1;
      e.cap_min_ival = cap_min_ival;
    } else {
      e.enabled = 0;
      e.cap_min_ival = 0;
    }
  }
  return AuxTraceInfoRecord(data, spe);
}

int SPERecorder::GetSPEEventType() {
  if (event_type_ == 0) {
    std::string s;
    std::string path = DEVICES_DIR + kSPEDefaultDeviceName + "/type";
    if (IsDir(path) || !android::base::ReadFileToString(path, &s) ||
        !android::base::ParseInt(android::base::Trim(s), &event_type_)) {
      event_type_ = -1;
    }
  }
  return event_type_;
}

uint64_t SPERecorder::GetMinInterval() {
  uint64_t cap_min_ival = 0;
  std::string min_interval;
  if (!android::base::ReadFileToString(DEVICES_DIR + kSPEDefaultDeviceName + "/caps/min_interval",
                                       &min_interval) ||
      !android::base::ParseUint(android::base::Trim(min_interval), &cap_min_ival)) {
    LOG(ERROR) << "Failed to read min interval from sysfs on path " << DEVICES_DIR
               << kSPEDefaultDeviceName << "/caps/min_interval";
  }
  return cap_min_ival;
}

bool SPERecorder::FindSpeConfig(const std::string& name, uint64_t* ret_val) {
  auto it = spe_config_.find(name);
  if (it != spe_config_.end()) {
    *ret_val = it->second;
    return true;
  } else {
    return false;
  }
}

}  // namespace simpleperf
