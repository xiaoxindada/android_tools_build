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

#include <stddef.h>

#include "report_utils.h"

namespace simpleperf {

// Wrapper class for simpleperf::ProguardMappingRetrace for python to call.
class ReportUtils {
 public:
  ProguardMappingRetrace retrace;
};

using ReportUtils = simpleperf::ReportUtils;

extern "C" {
#define EXPORT __attribute__((visibility("default")))

ReportUtils* ReportUtils_Create() EXPORT;
void ReportUtils_Destroy(ReportUtils* report_utils) EXPORT;

bool ReportUtils_AddProguardMappingFile(ReportUtils* report_utils, const char* mapping_file) EXPORT;
bool ReportUtils_DeObfuscateJavaMethods(const ReportUtils* report_utils,
                                        const char* obfuscated_name, char* original_name,
                                        size_t original_name_size) EXPORT;
}

ReportUtils* ReportUtils_Create() {
  return new ReportUtils();
}

void ReportUtils_Destroy(ReportUtils* report_utils) {
  delete report_utils;
}

bool ReportUtils_AddProguardMappingFile(ReportUtils* report_utils, const char* mapping_file) {
  return report_utils->retrace.AddProguardMappingFile(mapping_file);
}

bool ReportUtils_DeObfuscateJavaMethods(const ReportUtils* report_utils,
                                        const char* obfuscated_name, char* original_name,
                                        size_t original_name_size) {
  bool synthesized = false;
  std::string original_name_str = "";
  bool result = report_utils->retrace.DeObfuscateJavaMethods(obfuscated_name, &original_name_str,
                                                             &synthesized);
  if (result && original_name_size > 0) {
    size_t len = original_name_str.copy(original_name, original_name_size - 1);
    original_name[len] = '\0';
  }
  return result;
}

}  // namespace simpleperf
