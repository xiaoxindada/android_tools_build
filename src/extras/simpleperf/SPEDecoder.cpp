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

#include "SPEDecoder.h"
#include <android-base/memory.h>
#include <cstring>

namespace simpleperf {

// SPE packet header encodings
static constexpr uint8_t hdr_encoding_padding_msk = 0xff;
static constexpr uint8_t hdr_encoding_padding = 0x0;
static constexpr uint8_t hdr_encoding_end_pkt_msk = 0xff;
static constexpr uint8_t hdr_encoding_end_pkt = 0x1;
static constexpr uint8_t hdr_encoding_timestamp_pkt_msk = 0xff;
static constexpr uint8_t hdr_encoding_timestamp_pkt = 0x71;  // 0111 0001
static constexpr uint8_t hdr_encoding_events_pkt_msk = 0xcf;
static constexpr uint8_t hdr_encoding_events_pkt = 0x42;  // 01xx 0010
static constexpr uint8_t hdr_encoding_data_source_pkt_msk = 0xcf;
static constexpr uint8_t hdr_encoding_data_source_pkt = 0x43;  // 01xx 0011
static constexpr uint8_t hdr_encoding_context_pkt_msk = 0xfc;
static constexpr uint8_t hdr_encoding_context_pkt = 0x64;  // 0110 01xx
static constexpr uint8_t hdr_encoding_op_type_pkt_msk = 0xfc;
static constexpr uint8_t hdr_encoding_op_type_pkt = 0x48;  // 0100 10xx
static constexpr uint8_t hdr_encoding_address_pkt_msk = 0xf8;
static constexpr uint8_t hdr_encoding_address_pkt = 0xb0;  // 1011 0xxx
static constexpr uint8_t hdr_encoding_counter_pkt_msk = 0xf8;
static constexpr uint8_t hdr_encoding_counter_pkt = 0x98;  // 1001 1xxx

static constexpr uint8_t hdr_encoding_extended_hdr_msk = 0xE0;  // 1110 0000
static constexpr uint8_t hdr_encoding_extended_hdr = 0x20;      // 0010 0000 - 0011 1111

static constexpr uint8_t hdr_encoding_payload_size_msk = 0x30;  // bits 4 and 5
static constexpr size_t hdr_encoding_payload_size_shift = 4;
static constexpr size_t hdr_encoding_zero_payload_range_min = 0x00;
static constexpr size_t hdr_encoding_zero_payload_range_max = 0x1f;

// Header index defines for address type packets.
static constexpr int addr_pkt_type_instr_virt_addr = 0;
static constexpr int addr_pkt_type_br_target_addr = 1;
static constexpr int addr_pkt_type_data_virt_addr = 2;
static constexpr int addr_pkt_type_data_phys_addr = 3;
static constexpr int addr_pkt_type_prev_br_targ_addr = 4;
static constexpr uint64_t addr_pkt_payload_addr_byte7_shift = 56;
static constexpr uint64_t addr_pkt_payload_addr_msk =
    ~(0xffull << addr_pkt_payload_addr_byte7_shift);

static constexpr uint64_t addr_pkt_payload_misc_bits_shift = addr_pkt_payload_addr_byte7_shift;
static constexpr uint64_t addr_pkt_type_instr_virt_addr_misc_ns_msk = 0x1 << 7;
static constexpr uint64_t addr_pkt_type_instr_virt_addr_misc_el_msk = 0x3 << 5;
static constexpr uint64_t addr_pkt_type_instr_virt_addr_misc_nse_msk = 0x1 << 4;

// Header index defines for counter packets.
static constexpr uint8_t cnt_pkt_type_total_latency = 0x0;
static constexpr uint8_t cnt_pkt_type_issue_latency = 0x1;
static constexpr uint8_t cnt_pkt_type_translation_latency = 0x2;

// Payload event bits defined for event packets.
static constexpr uint64_t events_pkt_arch_retired = (1 << 1);
static constexpr uint64_t events_pkt_l1_dcache_access = (1 << 2);
static constexpr uint64_t events_pkt_l1_dcache_refill = (1 << 3);
static constexpr uint64_t events_pkt_tlb_access = (1 << 4);
static constexpr uint64_t events_pkt_tlb_walk = (1 << 5);
static constexpr uint64_t events_pkt_br_not_taken = (1 << 6);
static constexpr uint64_t events_pkt_br_mispred = (1 << 7);
static constexpr uint64_t events_pkt_llc_access = (1 << 8);
static constexpr uint64_t events_pkt_llc_miss = (1 << 9);
static constexpr uint64_t events_pkt_ld_st_remote_access = (1 << 10);
static constexpr uint64_t events_pkt_ld_st_misalignment = (1 << 11);
static constexpr uint64_t events_pkt_transactional = (1 << 16);
static constexpr uint64_t events_pkt_partial_or_empty_predicate = (1 << 17);
static constexpr uint64_t events_pkt_empty_predicate = (1 << 18);
static constexpr uint64_t events_pkt_l2_dcache_access = (1 << 19);         // needs FEAT_SPEv1p4
static constexpr uint64_t events_pkt_l2_dcache_miss = (1 << 20);           // needs FEAT_SPEv1p4
static constexpr uint64_t events_pkt_cache_data_modified = (1 << 21);      // needs FEAT_SPEv1p4
static constexpr uint64_t events_pkt_recently_fetched = (1 << 22);         // needs FEAT_SPEv1p4
static constexpr uint64_t events_pkt_data_snooped = (1 << 23);             // needs FEAT_SPEv1p4
static constexpr uint64_t events_pkt_streaming_sve_mode = (1 << 24);       // needs FEAT_SPEv1p4
static constexpr uint64_t events_pkt_ssmcu_or_coprocessor_op = (1 << 25);  // needs FEAT_SPEv1p4
static constexpr uint64_t events_pkt_supported_events_mask =
    (events_pkt_arch_retired | events_pkt_l1_dcache_access | events_pkt_l1_dcache_refill |
     events_pkt_tlb_access | events_pkt_tlb_walk | events_pkt_br_not_taken | events_pkt_br_mispred |
     events_pkt_llc_access | events_pkt_llc_miss | events_pkt_ld_st_remote_access |
     events_pkt_ld_st_misalignment | events_pkt_transactional |
     events_pkt_partial_or_empty_predicate | events_pkt_empty_predicate |
     events_pkt_l2_dcache_access | events_pkt_l2_dcache_miss | events_pkt_cache_data_modified |
     events_pkt_recently_fetched | events_pkt_data_snooped | events_pkt_streaming_sve_mode |
     events_pkt_ssmcu_or_coprocessor_op);

// In case of extended header the header byte should be byte 1.
// If the header byte is in the following range [0x00 - 0x1F], then payload size is 0.
// Otherwise bits 4 and 5 define the payload size.
size_t get_payload_size(uint8_t hdr) {
  if (hdr <= hdr_encoding_zero_payload_range_max) {
    return 0;
  }
  return (1ull << ((hdr & hdr_encoding_payload_size_msk) >> hdr_encoding_payload_size_shift));
}

// Used for Address and Counter type packets.
uint8_t get_index_bits(uint8_t* buf, bool extended_hdr) {
  uint8_t index = 0;
  if (extended_hdr) {
    index = ((buf[0] & 0x3) << 3) | (buf[1] & 0x7);
  } else {
    index = buf[0] & 0x7;
  }
  return index;
}

static size_t get_payload(uint8_t* buf, uint64_t* payload, size_t* size, bool* extended_hdr) {
  uint8_t hdr = *buf;
  if ((hdr & hdr_encoding_extended_hdr_msk) == hdr_encoding_extended_hdr) {
    *extended_hdr = true;
    if (*size > 1) {
      hdr = buf[1];
    }
  }
  size_t payload_size = get_payload_size(hdr);
  size_t hdr_size = (*extended_hdr ? 2 : 1);
  if (*size < (hdr_size + payload_size)) {
    LOG(ERROR) << "Truncated packet";
    *size = 0;
    return 0;
  }
  switch (payload_size) {
    case 0:
      break;
    case 1:
      *payload = *(buf + hdr_size);
      break;
    case 2:
      *payload = android::base::get_unaligned<uint16_t>(buf + hdr_size);
      break;
    case 4:
      *payload = android::base::get_unaligned<uint32_t>(buf + hdr_size);
      break;
    case 8:
      *payload = android::base::get_unaligned<uint64_t>(buf + hdr_size);
      break;
    default:
      LOG(ERROR) << "Invalid payload size!";
      *size = 0;
      break;
  }
  return payload_size;
}

constexpr std::string_view arm_spe_event_unused = "Unused";
constexpr std::string_view arm_spe_event_names[] = {arm_spe_event_unused,
                                                    "Architecturally retired",
                                                    "Level 1 data cache access",
                                                    "Level 1 data cache refill",
                                                    "TLB access",
                                                    "TLB walk",
                                                    "Condition not taken",
                                                    "Branch mispredicted",
                                                    "Last Level cache access",
                                                    "Last Level cache miss",
                                                    "Remote access",
                                                    "Misalignment",
                                                    arm_spe_event_unused,
                                                    arm_spe_event_unused,
                                                    arm_spe_event_unused,
                                                    arm_spe_event_unused,
                                                    "Operation executed in Transactional state",
                                                    "Partial or empty predicate",
                                                    "Empty predicate",
                                                    "Level 2 data cache access",
                                                    "Level 2 data cache miss",
                                                    "Cache data modified",
                                                    "Recently fetched",
                                                    "Data snooped",
                                                    "Streaming SVE mode",
                                                    "SMCU or external coprocessor operation"};

// Event ids are not continuous, so create a map that assigns a continues index
// to the event ids. It is used to map event attributes to event names.
constexpr std::array<size_t, ARM_SPE_EVENT_MAX> fill_event_index_map() {
  size_t event_attr_index = 0;
  std::array<size_t, ARM_SPE_EVENT_MAX> event_map = {};
  for (int i = 0; i < ARM_SPE_EVENT_MAX; i++) {
    if (arm_spe_event_names[i] != arm_spe_event_unused) {
      event_map[i] = event_attr_index;
      event_attr_index++;
    }
  }
  return event_map;
}
constexpr std::array<size_t, ARM_SPE_EVENT_MAX> event_index_map = fill_event_index_map();

// Replicate the single perf event attribute for SPE to have one for each SPE event.
// This function does not check the type of the attribute, caller has to make sure that
// the passed perf_event_attr has SPE type.
spe_perf_event_attr_with_name ReplicateSpeEventAttr(const perf_event_attr& attr) {
  spe_perf_event_attr_with_name spe_attr;
  for (int i = 0; i < ARM_SPE_EVENT_MAX; i++) {
    if (arm_spe_event_names[i] != arm_spe_event_unused) {
      spe_attr.attr.emplace_back(attr);
      std::string new_name = kSPEEventName + " - ";
      spe_attr.attr_name.emplace_back(new_name.append(arm_spe_event_names[i]));
    }
  }
  return spe_attr;
}

size_t GetSpeAttributeIndexFromEventId(size_t event_id) {
  if (event_id < ARM_SPE_EVENT_MAX) {
    return event_index_map[event_id];
  } else {
    LOG(DEBUG) << "Event id out of range: " << event_id;
    return std::numeric_limits<size_t>::max();
  }
}

std::unique_ptr<SPEDecoder> SPEDecoder::Create() {
  return std::make_unique<SPEDecoder>();
}

size_t SPEDecoder::decode_packet(uint8_t* buf, size_t* size) {
  uint8_t hdr = buf[0];
  bool extended_hdr = false;
  spe_packet spe_pkt = {};

  spe_pkt.sz = get_payload(buf, &spe_pkt.payload, size, &extended_hdr);
  if (*size == 0) {
    // Some error occurred in get_payload().
    return 0;
  }
  if (extended_hdr) {
    hdr = buf[1];
  }

  if ((hdr & hdr_encoding_padding_msk) == hdr_encoding_padding) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_PAD;
    // Several padding packets can follow each other. It is not worth creating an spe_pkt
    // struct for each padding packet, so just increase the size of this one.
    buf++;
    while ((*size > (spe_pkt.sz + 1)) && (buf[spe_pkt.sz] == 0) && (spe_pkt.sz < 16)) {
      spe_pkt.sz++;
    }
  } else if ((hdr & hdr_encoding_end_pkt_msk) == hdr_encoding_end_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_END;
  } else if ((hdr & hdr_encoding_timestamp_pkt_msk) == hdr_encoding_timestamp_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_TIMESTAMP;
  } else if ((hdr & hdr_encoding_events_pkt_msk) == hdr_encoding_events_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_EVENTS;
  } else if ((hdr & hdr_encoding_data_source_pkt_msk) == hdr_encoding_data_source_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_DATA_SOURCE;
  } else if ((hdr & hdr_encoding_context_pkt_msk) == hdr_encoding_context_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_CONTEXT;
    spe_pkt.misc = hdr & 0x3;
  } else if ((hdr & hdr_encoding_op_type_pkt_msk) == hdr_encoding_op_type_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_OP_TYPE;
    spe_pkt.misc = hdr & 0x3;
  } else if ((hdr & hdr_encoding_address_pkt_msk) == hdr_encoding_address_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_ADDRESS;
    spe_pkt.misc = get_index_bits(buf, extended_hdr);
  } else if ((hdr & hdr_encoding_counter_pkt_msk) == hdr_encoding_counter_pkt) {
    spe_pkt.type = ARM_SPE_PKT_TYPE_COUNTER;
    spe_pkt.misc = get_index_bits(buf, extended_hdr);
  } else {
    // Unsupported packets must be ignored, this is not an error case.
    LOG(DEBUG) << "Invalid or unsupported packet header: 0x" << std::hex << (unsigned int)hdr;
  }
  size_t packet_size = ((extended_hdr ? 2 : 1) + spe_pkt.sz);
  *size -= packet_size;
  spe_pkt_.push_back(spe_pkt);
  return packet_size;
}

// Replicate SpeSampleRecord for each event.
void CreateAndStoreSamplePerEvent(const perf_event_attr& attr, uint64_t spe_event_ids,
                                  uint64_t sample_type, uint64_t ip, uint64_t addr, uint32_t tid,
                                  const SampleId* sample_id, bool in_kernel,
                                  std::vector<SpeSampleRecord>& samples) {
  while (spe_event_ids) {
    size_t spe_event = GetSpeAttributeIndexFromEventId(std::countr_zero(spe_event_ids));
    if (spe_event != std::numeric_limits<size_t>::max()) {
      SpeSampleRecord new_sample(attr, sample_id->id_data.id, ip, sample_id->tid_data.pid, tid, 0,
                                 sample_id->cpu_data.cpu, 1, addr, {}, {}, {}, 0, in_kernel,
                                 sample_type, spe_event);
      samples.push_back(std::move(new_sample));
    }
    spe_event_ids &= (spe_event_ids - 1);
  }
}

std::vector<SpeSampleRecord> SPEDecoder::ProcessData(uint8_t* data, size_t size,
                                                     const SampleId* sample_id,
                                                     const perf_event_attr& attr) {
  bool new_sample = true;
  std::vector<SpeSampleRecord> samples = {};
  uint64_t spe_event_ids, sample_type, ip, addr;
  uint32_t tid;
  bool in_kernel;
  while (size) {
    // Read packets from the buffer into spe_pkt_.
    size_t ret = decode_packet(data, &size);
    data += ret;
  }
  // Parse through all the SPE packets, form records, and transform them into Sample type records.
  for (auto pkt : spe_pkt_) {
    if (new_sample) {
      spe_event_ids = 0;
      sample_type = 0;
      tid = sample_id->tid_data.tid;
      ip = 0;
      addr = 0;
      in_kernel = false;
      new_sample = false;
    }
    switch (pkt.type) {
      case ARM_SPE_PKT_TYPE_PAD:
        break;
      case ARM_SPE_PKT_TYPE_EVENTS:
        spe_event_ids = pkt.payload & events_pkt_supported_events_mask;
        break;
      case ARM_SPE_PKT_TYPE_DATA_SOURCE:
        // PERF_SAMPLE_DATA_SRC is not supported by Simpleperf.
        break;
      case ARM_SPE_PKT_TYPE_CONTEXT:
        tid = static_cast<uint32_t>(pkt.payload);
        break;
      case ARM_SPE_PKT_TYPE_OP_TYPE:
        // Not supported.
        break;
      case ARM_SPE_PKT_TYPE_ADDRESS: {
        switch (pkt.misc) {
          case addr_pkt_type_instr_virt_addr: {
            sample_type |= PERF_SAMPLE_IP;
            ip = pkt.payload & addr_pkt_payload_addr_msk;
            uint64_t misc = pkt.payload >> addr_pkt_payload_addr_byte7_shift;
            bool ns = (misc & addr_pkt_type_instr_virt_addr_misc_ns_msk);
            uint64_t el = (misc & addr_pkt_type_instr_virt_addr_misc_el_msk) >> 5;
            if (ns && ((el == 1u) || (el == 2u))) {
              ip |= (0xffull << addr_pkt_payload_addr_byte7_shift);
              in_kernel = true;
            }
            break;
          }
          case addr_pkt_type_br_target_addr: {
            // PERF_SAMPLE_BRANCH_STACK is not supported for SPE.
            break;
          }
          case addr_pkt_type_data_virt_addr: {
            sample_type |= PERF_SAMPLE_ADDR;
            addr = pkt.payload & addr_pkt_payload_addr_msk;
            break;
          }
          case addr_pkt_type_data_phys_addr: {
            // PERF_SAMPLE_PHYS_ADDR is not supported by Simpleperf.
            break;
          }
          case addr_pkt_type_prev_br_targ_addr: {
            // PERF_SAMPLE_BRANCH_STACK is not supported for SPE.
            break;
          }
          default:
            break;
        }
        break;
      }
      case ARM_SPE_PKT_TYPE_COUNTER:
        switch (pkt.misc) {
          case cnt_pkt_type_total_latency:
            // PERF_SAMPLE_WEIGHT is not supported by Simpleperf.
            break;
        }
        break;
      case ARM_SPE_PKT_TYPE_END:
      case ARM_SPE_PKT_TYPE_TIMESTAMP:
        // Timestamp is currently not used, just treat it as an END packet.
        CreateAndStoreSamplePerEvent(attr, spe_event_ids, sample_type, ip, addr, tid, sample_id,
                                     in_kernel, samples);
        new_sample = true;
        break;
      case ARM_SPE_PKT_TYPE_BAD:
        break;
      default:
        break;
    }
  }
  spe_pkt_.clear();
  return samples;
}

}  // namespace simpleperf
