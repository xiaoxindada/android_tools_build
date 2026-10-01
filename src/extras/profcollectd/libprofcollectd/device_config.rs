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

use anyhow::{Context, Result};
use log::{error, info};
use rustutils::android::system_properties;
use std::collections::HashMap;
use std::str;

/// Calls the config parser and then writes the resulting key-value pairs
/// to system properties using the (mocked) system_properties::write function.
pub fn write_config_to_properties() -> Result<()> {
    // Step 1. Read profcollect_native_boot device configs.
    let config = run_and_parse_config().context("Failed to run and parse device_config")?;

    // Step 2. Make sure device configs are synced to system properties. See b/433651427.
    for (key, value) in config.iter() {
        let property_name = format!("persist.device_config.profcollect_native_boot.{}", key);
        let property_value = system_properties::read(&property_name)?.unwrap_or("".to_string());
        if value != &property_value {
            info!("Updating property '{}' from '{}' to '{}'", property_name, property_value, value);
            system_properties::write(&property_name, value.as_str())?;
        }
    }

    // Step 3. Sync enabled device config to the system property starting profcollectd.
    let new_enabled = match config.get("enabled") {
        Some(original_value) => {
            // Key is present, check and normalize its value
            match original_value.to_lowercase().as_str() {
                "1" | "y" | "yes" | "on" | "true" => "true",
                "0" | "n" | "no" | "off" | "false" => "false",
                invalid => {
                    error!(
                        "Invalid value for 'enabled' device config: '{}'. Defaulting to 'false'.",
                        invalid
                    );
                    "false"
                }
            }
        }
        None => "false",
    };
    let old_enabled =
        system_properties::read("persist.profcollectd.enabled")?.unwrap_or("".to_string());

    if new_enabled != old_enabled {
        info!(
            "Updating 'persist.profcollectd.enabled' from '{}' to '{}'",
            old_enabled, new_enabled
        );
        system_properties::write("persist.profcollectd.enabled", new_enabled)?;
    }
    Ok(())
}

/// Executes the "device_config list profcollect_native_boot" command,
/// captures its stdout, and parses "key=value" lines into a HashMap.
///
/// On any error (IO, command failure, or decoding), it prints the error
/// to stderr and returns an empty HashMap.
fn run_and_parse_config() -> Result<HashMap<String, String>> {
    // 1. Execute the command and handle IO errors
    let args: Vec<&str> = vec!["device_config", "list", "profcollect_native_boot"];
    let output = simpleperf_profcollect::run_device_config_cmd(&args);

    // 2. Parse Lines into HashMap using iterator methods
    Ok(output
        .lines()
        // Trim whitespace and ignore empty lines
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            // Split the line at the first '='
            match trimmed.split_once('=') {
                Some((key_part, value_part)) => {
                    let key = key_part.trim().to_string();
                    let value = value_part.trim().to_string();

                    // Only map if the key is not empty
                    if key.is_empty() {
                        error!("Parsing Error: Skipped line with empty key: {}", trimmed);
                        None
                    } else {
                        Some((key, value))
                    }
                }
                None => {
                    // This line did not contain '=', violating the expected format.
                    // We print a warning and skip the line to handle minor formatting issues gracefully.
                    error!("Parsing Warning: Skipping line (missing '='): {}", trimmed);
                    None
                }
            }
        })
        // Collect the resulting iterator of (key, value) tuples into a HashMap
        .collect())
}
