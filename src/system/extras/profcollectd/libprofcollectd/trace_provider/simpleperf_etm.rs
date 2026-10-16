//
// Copyright (C) 2021 The Android Open Source Project
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

//! Trace provider backed by ARM Coresight ETM, using simpleperf tool.

use anyhow::{anyhow, Result};
use nix::libc::{sysconf, _SC_PAGESIZE};
use std::fs::{read_dir, remove_file, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Duration;
use trace_provider::{prune_memory, TraceProvider};

use crate::config;
use crate::trace_provider;

static ETM_TRACEFILE_EXTENSION: &str = "etmtrace";
static ETM_PROFILE_EXTENSION: &str = "data";

struct BufferConfig {
    user_buffer_size: u64, // Bytes
    aux_buffer_size: u64,  // Bytes
    mmap_pages: u64,       // Page count
}

/// Helper to get system page size safely (defaults to 4096 on failure)
fn get_system_page_size() -> u64 {
    // SAFETY: `sysconf` is safe to call with `_SC_PAGESIZE`.
    unsafe {
        let val = sysconf(_SC_PAGESIZE);
        if val > 0 {
            val as u64
        } else {
            4096 // Fallback standard 4KB
        }
    }
}

/// Helper to calculate pages needed to hit a target byte size.
/// Ensures the result is at least 1 page, and that target_bytes is a multiple of page_size.
fn pages_for_bytes(target_bytes: u64, page_size: u64) -> u64 {
    assert_eq!(
        target_bytes % page_size,
        0,
        "target_bytes ({}) must be a multiple of page_size ({})",
        target_bytes,
        page_size
    );
    let pages = target_bytes / page_size;
    if pages == 0 {
        1
    } else {
        pages
    }
}

fn get_mem_available_kb() -> Result<u64, std::io::Error> {
    let path = Path::new("/proc/meminfo");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("MemAvailable:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(val) = parts[1].parse::<u64>() {
                    return Ok(val);
                }
            }
        }
    }
    Ok(0)
}

fn get_memory_adaptive_buffer_config() -> Result<BufferConfig> {
    // 1. Get System Stats
    let mem_available_kb = get_mem_available_kb().unwrap_or(0);
    let mut mem_available_mb = mem_available_kb / 1024;
    let page_size = get_system_page_size();

    let mem_available_mb_prop =
        config::get_device_config("mem_available_mb", "".to_string()).unwrap_or("".to_string());
    if !mem_available_mb_prop.is_empty() {
        mem_available_mb = mem_available_mb_prop.parse::<u64>().unwrap_or(mem_available_mb);
        log::info!("Use mem_avail_mb from device_config: {}MB", mem_available_mb);
    }

    // 2. Apply Tier Logic
    // Memory tiering for buffer configuration (assuming 6 CPUs (middle and big cores on Pixel 10)
    // for estimation):
    // TIER 0: SKIP (< 256MB). Not enough memory.
    // TIER 1: Survival (256MB - 511MB). User buffer: 8MB, Aux buffer: 0.5MB. Mmap buffer: 0.25MB.
    //         Total 8+0.75*cpus = 12.5MB.
    // TIER 2: Low (512MB - 1023MB). User buffer: 16MB, Aux buffer: 1MB. Mmap buffer: 0.25MB.
    //         Total 16+1.25*cpus = 23.5MB.
    // TIER 3: Medium (1GB - 2GB). User buffer: 64MB, Aux buffer: 2MB. Mmap buffer: 0.5MB.
    //         Total 64+2.5*cpus = 79MB.
    // TIER 4: High (2GB - 3GB). User buffer: 128MB, Aux buffer: 4MB. Mmap buffer: 1MB.
    //         Total 128+5*cpus=158MB.
    // TIER 5: Max (> 3GB). User buffer: 256MB, Aux buffer: 4MB. Mmap buffer: 4MB.
    //         Total 256+8*cpus=304MB.
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    let (user, aux, mmap_target) = match mem_available_mb {
        0..=255 => return Err(anyhow!("not enough mem_avail")),
        256..=511 => (8 * MB, 512 * KB, 256 * KB),
        512..=1023 => (16 * MB, MB, 256 * KB),
        1024..=2047 => (64 * MB, 2 * MB, 512 * KB),
        2048..=3071 => (128 * MB, 4 * MB, MB),
        _ => (256 * MB, 4 * MB, 4 * MB),
    };

    Ok(BufferConfig {
        user_buffer_size: user,
        aux_buffer_size: aux,
        mmap_pages: pages_for_bytes(mmap_target, page_size),
    })
}

pub struct SimpleperfEtmTraceProvider {}

impl TraceProvider for SimpleperfEtmTraceProvider {
    fn get_name(&self) -> &'static str {
        "simpleperf_etm"
    }

    fn is_ready(&self) -> bool {
        simpleperf_profcollect::is_etm_device_available()
    }

    fn trace_system(
        &self,
        trace_dir: &Path,
        tag: &str,
        sampling_period: &Duration,
        binary_filter: &str,
    ) {
        let trace_file = trace_provider::get_path(trace_dir, tag, ETM_TRACEFILE_EXTENSION);
        // Record ETM data for kernel space only when it's not filtered out by binary_filter. So we
        // can get more ETM data for user space when ETM data for kernel space isn't needed.
        let event_name = if binary_filter.contains("kernel") { "cs-etm" } else { "cs-etm:u" };
        let duration: String = sampling_period.as_secs_f64().to_string();
        let buffer_config = match get_memory_adaptive_buffer_config() {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to get memory adaptive buffer config: {}", e);
                return;
            }
        };
        let user_buffer_size_str = buffer_config.user_buffer_size.to_string();
        let aux_buffer_size_str = buffer_config.aux_buffer_size.to_string();
        let mmap_pages_str = buffer_config.mmap_pages.to_string();
        let args: Vec<&str> = vec![
            "-a",
            "-e",
            event_name,
            "--duration",
            &duration,
            "-z",
            "--binary",
            binary_filter,
            "--no-dump-build-id",
            "--no-dump-symbols",
            "--no-dump-kernel-symbols",
            "--user-buffer-size",
            &user_buffer_size_str,
            "--aux-buffer-size",
            &aux_buffer_size_str,
            "-m",
            &mmap_pages_str,
            "-o",
            trace_file.to_str().unwrap(),
        ];
        simpleperf_profcollect::run_record_cmd(&args);
    }

    fn trace_process(
        &self,
        trace_dir: &Path,
        tag: &str,
        sampling_period: &Duration,
        processes: &str,
    ) {
        let trace_file = trace_provider::get_path(trace_dir, tag, ETM_TRACEFILE_EXTENSION);
        let event_name = "cs-etm:u";
        let duration: String = sampling_period.as_secs_f64().to_string();
        let buffer_config = match get_memory_adaptive_buffer_config() {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to get memory adaptive buffer config: {}", e);
                return;
            }
        };
        let user_buffer_size_str = buffer_config.user_buffer_size.to_string();
        let aux_buffer_size_str = buffer_config.aux_buffer_size.to_string();
        let mmap_pages_str = buffer_config.mmap_pages.to_string();
        let args: Vec<&str> = vec![
            "-p",
            processes,
            "-e",
            event_name,
            "--duration",
            &duration,
            "-z",
            "--no-dump-symbols",
            "--user-buffer-size",
            &user_buffer_size_str,
            "--aux-buffer-size",
            &aux_buffer_size_str,
            "-m",
            &mmap_pages_str,
            "-o",
            trace_file.to_str().unwrap(),
        ];
        simpleperf_profcollect::run_record_cmd(&args);
    }

    fn process(&self, trace_dir: &Path, profile_dir: &Path, binary_filter: &str) -> Result<()> {
        let is_etm_extension = |file: &PathBuf| {
            file.extension()
                .and_then(|f| f.to_str())
                .filter(|ext| ext == &ETM_TRACEFILE_EXTENSION)
                .is_some()
        };

        let process_trace_file = |trace_file: PathBuf| -> Result<()> {
            let mut profile_file = PathBuf::from(profile_dir);
            profile_file.push(
                trace_file
                    .file_name()
                    .ok_or_else(|| anyhow!("Malformed trace path: {}", trace_file.display()))?,
            );
            profile_file.set_extension(ETM_PROFILE_EXTENSION);

            let args: Vec<&str> = vec![
                "-i",
                trace_file.to_str().unwrap(),
                "-o",
                profile_file.to_str().unwrap(),
                "--output",
                "branch-list",
                "--binary",
                binary_filter,
                "--exclude-perf",
            ];
            simpleperf_profcollect::run_inject_cmd(&args);
            remove_file(&trace_file)?;
            Ok(())
        };

        read_dir(trace_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|e| e.is_file())
            .filter(is_etm_extension)
            .try_for_each(process_trace_file)?;

        prune_memory();
        Ok(())
    }

    fn set_log_file(&self, filename: &Path) {
        simpleperf_profcollect::set_log_file(filename);
    }

    fn reset_log_file(&self) {
        simpleperf_profcollect::reset_log_file();
    }
}

impl SimpleperfEtmTraceProvider {
    pub fn supported() -> bool {
        simpleperf_profcollect::is_etm_driver_available()
    }
}
