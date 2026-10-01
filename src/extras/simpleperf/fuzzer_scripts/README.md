# Simpleperf Fuzzer Scripts

This directory contains scripts and tools for debugging and reproducing issues
found by fuzzers in `simpleperf`.

## Contents

-   `repro_fuzz_issue.sh`: A comprehensive script to automate the build, sync,
    and execution of the `libsimpleperf_report_fuzzer` on an Android device.

## Prerequisites

Before running any scripts, ensure your environment is set up for Android
platform development:

1.  **Initialize the environment:** `bash source build/envsetup.sh`
2.  **Select a target:** `bash lunch <your_target>`
3.  **Connect a device:** Ensure a device or emulator is connected via ADB.

## Usage: `repro_fuzz_issue.sh`

The `repro_fuzz_issue.sh` script automates the process of building the fuzzer,
pushing the necessary binaries and libraries to the device, and running a
specific test case.

### Running with a test case

To reproduce a crash or issue found by a fuzzer:

```bash
./repro_fuzz_issue.sh /path/to/repro_test_case
```

The script will:

1.  Build `libsimpleperf_report_fuzzer` for your current `lunch` target.
2.  Create `/data/fuzzer` on the device.
3.  Sync the fuzzer binary and its required shared libraries to the device.
4.  Push the specific test case to the device.
5.  Execute the fuzzer on the device using the provided testcase with the
    correct `LD_LIBRARY_PATH`.

### Syncing binaries only

If you want to build and sync the binaries without running a specific test case:
`./repro_fuzz_issue.sh`

## Device Paths

-   **Binaries & Libraries:** `/data/fuzzer`
-   **Test cases:** `/data/fuzzer/testcase`

