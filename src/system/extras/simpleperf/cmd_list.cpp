/*
 * Copyright (C) 2015 The Android Open Source Project
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

#include <sched.h>
#include <stdio.h>

#include <atomic>
#include <format>
#include <map>
#include <print>
#include <string>
#include <thread>
#include <vector>

#include <android-base/file.h>
#include <android-base/logging.h>
#include <android-base/strings.h>

#include "ETMRecorder.h"
#include "RegEx.h"
#include "SPERecorder.h"
#include "command.h"
#include "environment.h"
#include "event_attr.h"
#include "event_fd.h"
#include "event_selection_set.h"
#include "event_type.h"
#include "utils.h"

namespace simpleperf {

extern std::unordered_map<std::string, std::unordered_set<int>> cpu_supported_raw_events;

#if defined(__aarch64__) || defined(__arm__)
extern std::unordered_map<uint64_t, std::string> cpuid_to_name;
#endif  // defined(__aarch64__) || defined(__arm__)

#if defined(__riscv)
extern std::map<std::tuple<uint64_t, std::string, std::string>, std::string> cpuid_to_name;
#endif  // defined(__riscv)

namespace {

struct RawEventTestThreadArg {
  int cpu;
  std::atomic<pid_t> tid;
  std::atomic<bool> start;
};

void RawEventTestThread(RawEventTestThreadArg* arg) {
  cpu_set_t mask;
  CPU_ZERO(&mask);
  CPU_SET(arg->cpu, &mask);

  // Store TID and notify the main thread that we are ready
  arg->tid.store(gettid(), std::memory_order_release);

  if (sched_setaffinity(0, sizeof(mask), &mask) != 0) {
    return;
  }

  arg->start.wait(false, std::memory_order_acquire);

  TemporaryFile tmpfile;
  auto fp = std::unique_ptr<FILE, decltype(&fclose)>(fopen(tmpfile.path, "w"), fclose);
  if (fp) {
    for (int i = 0; i < 10; ++i) {
      std::print(fp.get(), "output some data to trigger pmu\n");
    }
  }
}

struct RawEventSupportStatus {
  std::set<int> supported_cpus;
  std::set<int> may_supported_cpus;
};

#if defined(__riscv)
std::string to_hex_string(uint64_t value) {
  std::stringstream stream;
  stream << "0x" << std::hex << value;
  return stream.str();
}

auto find_cpu_name(
    const std::tuple<uint64_t, uint64_t, uint64_t>& cpu_id,
    const std::map<std::tuple<uint64_t, std::string, std::string>, std::string>& cpuid_to_name) {
  // cpu_id: mvendorid, marchid, mimpid
  // cpuid_to_name: mvendorid, marchid regex, mimpid regex

  std::string marchid_hex = to_hex_string(get<1>(cpu_id));
  std::string mimpid_hex = to_hex_string(get<2>(cpu_id));
  uint64_t mvendorid = std::get<0>(cpu_id);

  // Search the first entry that matches mvendorid
  auto it = cpuid_to_name.lower_bound({mvendorid, "", ""});

  // Search the iterator of correct regex for current CPU from entries with same mvendorid
  for (; it != cpuid_to_name.end() && std::get<0>(it->first) == mvendorid; ++it) {
    const auto& [_, marchid_regex, mimpid_regex] = it->first;
    if (RegEx::Create(marchid_regex)->Match(marchid_hex) &&
        RegEx::Create(mimpid_regex)->Match(mimpid_hex)) {
      break;
    }
  }

  return it;
}
#endif  // defined(__riscv)

class RawEventSupportChecker {
 public:
  bool Init() {
    cpu_models_ = GetCpuModels();
    if (cpu_models_.empty()) {
      LOG(ERROR) << "can't get device cpu info";
      return false;
    }
    for (const auto& model : cpu_models_) {
      cpu_model_names_.push_back(GetCpuModelName(model));
    }
    return true;
  }

  RawEventSupportStatus GetCpusSupportingEvent(const EventType& event_type) {
    RawEventSupportStatus status;
    std::string required_cpu_model;
    // For cpu model specific events, the limited_arch is like "arm64:Cortex-A520".
    if (auto pos = event_type.limited_arch.find(':'); pos != std::string::npos) {
      required_cpu_model = event_type.limited_arch.substr(pos + 1);
    }

    for (size_t i = 0; i < cpu_models_.size(); ++i) {
      const CpuModel& model = cpu_models_[i];
      const std::string& model_name = cpu_model_names_[i];
      bool got_status = false;
      bool supported = false;
      bool may_supported = false;

      if (model.arch == "arm") {
        if (!required_cpu_model.empty()) {
          // This is a cpu model specific event, only supported on required_cpu_model.
          supported = model_name == required_cpu_model;
          got_status = true;
        } else if (!model_name.empty()) {
          // We know events supported on this cpu model.
          auto it = cpu_supported_raw_events.find(model_name);
          CHECK(it != cpu_supported_raw_events.end())
              << "no events configuration for " << model_name;
          supported = it->second.count(event_type.config) > 0;
          got_status = true;
        }
      } else if (model.arch == "x86") {
        std::string limited_arch = event_type.limited_arch;
        if (auto pos = limited_arch.find(':'); pos != std::string::npos) {
          limited_arch = limited_arch.substr(0, pos);
        }
        if (limited_arch != model_name) {
          supported = false;
          got_status = true;
        }
      }

      if (!got_status) {
        // We need to test the event support status.
        TestEventSupportOnCpu(event_type, model.cpus[0], supported, may_supported);
      }

      if (supported) {
        status.supported_cpus.insert(model.cpus.begin(), model.cpus.end());
      } else if (may_supported) {
        status.may_supported_cpus.insert(model.cpus.begin(), model.cpus.end());
      }
    }
    return status;
  }

 private:
  std::string GetCpuModelName(const CpuModel& model) {
#if defined(__aarch64__) || defined(__arm__)
    uint64_t cpu_id =
        (static_cast<uint64_t>(model.arm_data.implementer) << 32) | model.arm_data.partnum;
    auto it = cpuid_to_name.find(cpu_id);
    if (it != cpuid_to_name.end()) {
      return it->second;
    }
#elif defined(__riscv)
    std::tuple<uint64_t, uint64_t, uint64_t> cpu_id = {
        model.riscv_data.mvendorid, model.riscv_data.marchid, model.riscv_data.mimpid};
    auto it = find_cpu_name(cpu_id, cpuid_to_name);
    if (it != cpuid_to_name.end()) {
      return it->second;
    }
#elif defined(__i386__) || defined(__x86_64__)
    if (android::base::StartsWith(model.x86_data.vendor_id, "GenuineIntel")) {
      return "x86-intel";
    }
    if (android::base::StartsWith(model.x86_data.vendor_id, "AuthenticAMD")) {
      return "x86-amd";
    }
#endif  // defined(__i386__) || defined(__x86_64__)
    return "";
  }

  void TestEventSupportOnCpu(const EventType& event_type, int cpu, bool& supported,
                             bool& may_supported) {
    // 1. Prepare Thread Arguments
    RawEventTestThreadArg test_thread_arg{.cpu = cpu, .tid = 0, .start = false};

    std::thread test_thread(RawEventTestThread, &test_thread_arg);

    // 2. Wait for TID with yield instead of fixed sleep (lower latency)
    while (test_thread_arg.tid.load(std::memory_order_acquire) == 0) {
      std::this_thread::yield();
    }

    // 3. Setup Perf Event
    perf_event_attr attr = CreateDefaultPerfEventAttr(event_type);
    attr.exclude_kernel = 1;

#if defined(__i386__) || defined(__x86_64__)
    // Handle Intel hybrid architectures (Performance vs. Efficiency cores)
    if (GetX86IntelAtomCpus().contains(cpu)) {
      attr.config = event_type.GetIntelAtomCpuConfig();
    }
#endif

    // Open the counter on the specific thread and CPU
    std::unique_ptr<EventFd> event_fd =
        EventFd::OpenEventFile(attr, test_thread_arg.tid.load(std::memory_order_acquire),
                               test_thread_arg.cpu, nullptr, event_type.name, false);

    // Trigger Work and Collect Data
    test_thread_arg.start.store(true, std::memory_order_release);

    test_thread_arg.start.notify_one();

    test_thread.join();

    // Evaluate Support
    supported = false;
    may_supported = false;

    if (event_fd != nullptr) {
      PerfCounter counter;
      if (event_fd->ReadCounter(&counter)) {
        if (counter.value != 0) {
          supported = true;  // Verified: Hardware actually counted something
        } else {
          may_supported = true;  // Kernel allowed it, but 0 events were recorded
        }
      }
    }
  }

  std::vector<CpuModel> cpu_models_;

  std::vector<std::string> cpu_model_names_;
};

void PrintRawEventTypes(std::string_view type_desc) {
  std::print("List of {}:\n", type_desc);

#if defined(__aarch64__) || defined(__arm__)
  std::print(R"(  # Please refer to "PMU common architectural and microarchitectural event numbers"
  # and "ARM recommendations for IMPLEMENTATION DEFINED event numbers" listed in
  # ARMv9 manual for details.
  # A possible link is https://developer.arm.com/documentation/ddi0487.
)");
#endif

  RawEventSupportChecker support_checker;
  if (!support_checker.Init()) {
    return;
  }

  auto callback = [&](const EventType& event_type) -> bool {
    if (event_type.type != PERF_TYPE_RAW) {
      return true;
    }

    RawEventSupportStatus status = support_checker.GetCpusSupportingEvent(event_type);
    if (status.supported_cpus.empty() && status.may_supported_cpus.empty()) {
      return true;
    }

    // 1. Build the CPU support string dynamically
    std::string cpu_info;
    if (!status.supported_cpus.empty()) {
      cpu_info = std::format("supported on cpu {}", ToCpuString(status.supported_cpus));
    }
    if (!status.may_supported_cpus.empty()) {
      if (!cpu_info.empty()) cpu_info += ", ";
      cpu_info += std::format("may supported on cpu {}", ToCpuString(status.may_supported_cpus));
    }

    // 2. Combine name and cpu_info for the left column
    std::string label = std::format("  {} ({})", event_type.name, cpu_info);

    // 3. Print with aligned columns
    // We use a width of 60 for the label, then append the comment
    if (!event_type.description.empty()) {
      std::print("{:<64} # {} (Event 0x{:04x})\n", label, event_type.description,
                 event_type.config);
    } else {
      std::print("{}\n", label);
    }

    return true;
  };

  EventTypeManager::Instance().ForEachType(callback);
  std::print("\n");
}

bool IsEventTypeSupported(const EventType& event_type) {
  // PMU and tracepoint events are provided by kernel. So we assume they're supported.
  if (event_type.IsPmuEvent() || event_type.IsTracepointEvent()) {
    return true;
  }
  perf_event_attr attr = CreateDefaultPerfEventAttr(event_type);
  // Exclude kernel to list supported events even when kernel recording isn't allowed.
  attr.exclude_kernel = 1;
  return IsEventAttrSupported(attr, event_type.name);
}

static void PrintEventTypesOfType(std::string_view type_name, std::string_view type_desc,
                                  const std::function<bool(const EventType&)>& is_type_fn) {
  if (type_name == "raw") {
    return PrintRawEventTypes(type_desc);
  }

  std::print("List of {}:\n", type_desc);

  // Architecture-specific hints
  if (GetTargetArch() == ARCH_ARM || GetTargetArch() == ARCH_ARM64) {
    if (type_name == "cache") {
      std::print("  # More cache events are available in `simpleperf list raw`.\n");
    }
  }

  auto callback = [&](const EventType& event_type) -> bool {
    if (!is_type_fn(event_type) || !IsEventTypeSupported(event_type)) {
      return true;
    }

    // Consistent alignment: we use the same 60-character width as PrintRawEventTypes
    if (!event_type.description.empty()) {
      std::print("  {:<58} # {}\n", event_type.name, event_type.description);
    } else {
      std::print("  {}\n", event_type.name);
    }

    return true;
  };

  EventTypeManager::Instance().ForEachType(callback);
  std::print("\n");
}

class ListCommand : public Command {
 public:
  ListCommand()
      : Command("list", "list available event types",
                // clang-format off
"Usage: simpleperf list [options] [hw|sw|cache|raw|tracepoint|arm_spe|pmu]\n"
"       List all available event types.\n"
"       Filters can be used to show only event types belong to selected types:\n"
"         hw          hardware events\n"
"         sw          software events\n"
"         cache       hardware cache events\n"
"         raw         raw cpu pmu events\n"
"         tracepoint  tracepoint events\n"
"         cs-etm      coresight etm instruction tracing events\n"
"         arm_spe     arm statistical profiling extension\n"
"         pmu         system-specific pmu events\n"
"Options:\n"
"--show-features    Show features supported on the device, including:\n"
"                     dwarf-based-call-graph\n"
"                     trace-offcpu\n"
                // clang-format on
        ) {}

  bool Run(const std::vector<std::string>& args) override {
    if (!CheckPerfEventLimit()) {
      return false;
    }

    static std::map<std::string, std::pair<std::string, std::function<bool(const EventType&)>>>
        type_map = {
            {"hw",
             {"hardware events", [](const EventType& e) { return e.type == PERF_TYPE_HARDWARE; }}},
            {"sw",
             {"software events", [](const EventType& e) { return e.type == PERF_TYPE_SOFTWARE; }}},
            {"cache",
             {"hw-cache events", [](const EventType& e) { return e.type == PERF_TYPE_HW_CACHE; }}},
            {"raw",
             {"raw events provided by cpu pmu",
              [](const EventType& e) { return e.type == PERF_TYPE_RAW; }}},
            {"tracepoint",
             {"tracepoint events",
              [](const EventType& e) { return e.type == PERF_TYPE_TRACEPOINT; }}},
#if defined(__arm__) || defined(__aarch64__)
            {"cs-etm",
             { "coresight etm events",
               [](const EventType& e) {
                 return e.type == ETMRecorder::GetInstance().GetEtmEventType();
               } }},
            {"arm_spe",
             { "Arm Statistical Profiling Extension events",
               [](const EventType& e) {
                 return e.type == SPERecorder::GetInstance().GetSPEEventType();
               } }},
#endif
            {"pmu",
             {"pmu events",
              [](const EventType& e) { return (e.IsPmuEvent() && !e.IsSpeEvent()); }}},
        };

    std::vector<std::string> names;
    if (args.empty()) {
      for (auto& item : type_map) {
        names.push_back(item.first);
      }
    } else {
      for (auto& arg : args) {
        if (type_map.find(arg) != type_map.end()) {
          names.push_back(arg);
        } else if (arg == "--show-features") {
          ShowFeatures();
          return true;
        } else {
          LOG(ERROR) << "unknown event type category: " << arg << ", try using \"help list\"";
          return false;
        }
      }
    }

    for (auto& name : names) {
      auto it = type_map.find(name);
      PrintEventTypesOfType(name, it->second.first, it->second.second);
    }
    return true;
  }

 private:
  void ShowFeatures() {
    if (IsDwarfCallChainSamplingSupported()) {
      std::print("dwarf-based-call-graph\n");
    }
    if (IsDumpingRegsForTracepointEventsSupported()) {
      std::print("trace-offcpu\n");
    }
    if (IsSettingClockIdSupported()) {
      std::print("set-clockid\n");
    }
  }
};
}  // namespace

void RegisterListCommand() {
  RegisterCommand("list", [] { return std::make_unique<ListCommand>(); });
}

}  // namespace simpleperf
