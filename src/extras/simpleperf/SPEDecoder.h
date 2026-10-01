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

#include <cstddef>
#include <cstdint>
#include <memory>

#include "event_type.h"
#include "record.h"
#include "thread_tree.h"

namespace simpleperf {

static constexpr size_t ARM_SPE_EVENT_MAX = 26;

enum arm_spe_pkt_type {
  ARM_SPE_PKT_TYPE_PAD,
  ARM_SPE_PKT_TYPE_END,
  ARM_SPE_PKT_TYPE_TIMESTAMP,
  ARM_SPE_PKT_TYPE_EVENTS,
  ARM_SPE_PKT_TYPE_DATA_SOURCE,
  ARM_SPE_PKT_TYPE_CONTEXT,
  ARM_SPE_PKT_TYPE_OP_TYPE,
  ARM_SPE_PKT_TYPE_ADDRESS,
  ARM_SPE_PKT_TYPE_COUNTER,
  ARM_SPE_PKT_TYPE_BAD,
};

struct spe_perf_event_attr_with_name {
  std::vector<struct perf_event_attr> attr;
  std::vector<std::string> attr_name;
};

struct spe_packet {
  enum arm_spe_pkt_type type;
  uint64_t payload;
  size_t sz;     // payload size in bytes
  uint8_t misc;  // specific data, needed for some type of the packets
};

// Sample Record formed from SPE packets.
struct SpeSampleRecord : public SampleRecord {
 public:
  SpeSampleRecord() : spe_event_id_(0){};
  SpeSampleRecord(const perf_event_attr& attr, uint64_t id, uint64_t ip, uint32_t pid, uint32_t tid,
                  uint64_t time, uint32_t cpu, uint64_t period, uint64_t addr,
                  const PerfSampleReadType& read_data, const std::vector<uint64_t>& ips,
                  const std::vector<char>& stack, uint64_t dyn_stack_size, bool in_kernel,
                  uint64_t additional_sample_type, uint64_t spe_event_id)
      : SampleRecord(attr, id, ip, pid, tid, time, cpu, period, addr, read_data, ips, stack,
                     dyn_stack_size, in_kernel, additional_sample_type),
        spe_event_id_(spe_event_id){};
  uint64_t GetSpeEventId() const { return spe_event_id_; };
  void SetSpeEventId(uint64_t event_id) { spe_event_id_ = event_id; }

 private:
  uint64_t spe_event_id_;  // Single event id after the Sample is replicated for all events.
};

spe_perf_event_attr_with_name ReplicateSpeEventAttr(const perf_event_attr& attr);
size_t GetSpeAttributeIndexFromEventId(size_t event_id);

class SPEDecoder {
 public:
  SPEDecoder(){};
  static std::unique_ptr<SPEDecoder> Create();
  std::vector<SpeSampleRecord> ProcessData(uint8_t* data, size_t size, const SampleId* sample_id,
                                           const perf_event_attr& attr);

 private:
  size_t decode_packet(uint8_t* buf, size_t* size);
  std::vector<spe_packet> spe_pkt_;
};

}  // namespace simpleperf
