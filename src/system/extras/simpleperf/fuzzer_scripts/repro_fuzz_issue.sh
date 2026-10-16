#!/bin/bash

# Exit on error, undefined vars, and pipe failures
set -euo pipefail

# --- Functions ---

show_help() {
  cat << EOF
Usage: ${0##*/} [testcase_path]

Automates building, syncing, and running an Android fuzzer. If a testcase_path
is provided, the script will push the testcase to the device and run the fuzzer
on the testcase. Otherwise, the script will only build and sync the binaries.

Options:
  -h, --help    Show this help message and exit

Environment Requirements:
  Must be run from within an AOSP repo after 'source build/envsetup.sh' and 'lunch'.
EOF
  exit 0
}

# --- Argument Parsing ---

# Check for help flag
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  show_help
fi

# --- Configuration ---
readonly REPO_ROOT="${ANDROID_BUILD_TOP:-}"

if [[ -z "$REPO_ROOT" ]]; then
  printf "Error: ANDROID_BUILD_TOP not set. Did you run 'source build/envsetup.sh'?\n"
  exit 1
fi

readonly FUZZER_TARGET="libsimpleperf_report_fuzzer"
readonly REMOTE_DIR="/data/fuzzer"

# --- Pre-flight Checks ---
# Check if adb is available and a device is connected
if ! command -v adb &>/dev/null; then
  printf "Error: 'adb' command not found. Is the Android SDK in your PATH?\n"
  exit 1
fi

if ! adb get-state &>/dev/null; then
  printf "Error: No device connected via ADB.\n"
  exit 1
fi

# Ensure TARGET_PRODUCT is set (usually from 'lunch')
if [[ -z "${TARGET_PRODUCT:-}" ]]; then
  printf "Error: TARGET_PRODUCT is not set. Did you run 'lunch'?\n"
  exit 1
fi

# Capture device info
readonly RAW_ARCH=$(adb shell getprop ro.product.cpu.abi | tr -d '\r')

if [[ "$RAW_ARCH" == "arm64-v8a" || "$RAW_ARCH" == "aarch64" ]]; then
  readonly ARCH="arm64"
else
  readonly ARCH="$RAW_ARCH"
fi

# Use the local TARGET_PRODUCT for paths to ensure consistency with the build system
PRODUCT="$TARGET_PRODUCT"

# Use vsoc_x86_64 for aosp_cf_x86_64_phone to ensure consistency with the build system
if [[ "$PRODUCT" == "aosp_cf_x86_64_phone" ]]; then
  PRODUCT="vsoc_x86_64"
fi

# Define local build paths for readability
readonly LOCAL_FUZZ_DIR="$REPO_ROOT/out/target/product/$PRODUCT/symbols/data/fuzz/$ARCH"

# --- Build Phase ---
printf "> Building %s for %s (%s)...\n" "$FUZZER_TARGET" "$PRODUCT" "$ARCH"
ANDROID_QUIET_BUILD=true m "$FUZZER_TARGET"

# --- Sync Phase ---
printf "> Preparing device directories...\n"
adb shell "mkdir -p $REMOTE_DIR/testcase $REMOTE_DIR/lib"

printf "> Syncing binaries...\n"
# Verify the local binary exists before pushing
if [[ ! -f "$LOCAL_FUZZ_DIR/$FUZZER_TARGET/$FUZZER_TARGET" ]]; then
  printf "Error: Could not find built binary at %s\n" "$LOCAL_FUZZ_DIR/$FUZZER_TARGET/$FUZZER_TARGET"
  exit 1
fi

adb push --sync "$LOCAL_FUZZ_DIR/lib/." "$REMOTE_DIR/lib/"
adb push "$LOCAL_FUZZ_DIR/$FUZZER_TARGET/$FUZZER_TARGET" "$REMOTE_DIR/"

# --- Execution Phase ---
if [[ -n "${1:-}" ]]; then
  readonly TESTCASE_PATH="$1"
  readonly TESTCASE="${TESTCASE_PATH##*/}"
  readonly local_cmd="LD_LIBRARY_PATH=$REMOTE_DIR/lib $REMOTE_DIR/$FUZZER_TARGET $REMOTE_DIR/testcase/$TESTCASE"

  printf "> Pushing testcase: %s\n" "$TESTCASE"
  adb push "$TESTCASE_PATH" "$REMOTE_DIR/testcase/"

  printf "> Clearing logcat...\n"
  adb logcat -c
  printf "logcat cleared.\n"
  printf "You can use \`adb logcat | grep \"libsimpleperf_report_fuzzer\"\` to view fuzzer logs.\n"

  printf "> Running fuzzer...\n"
  # Using 'set -x' inside the adb shell can help debug execution issues
  adb shell "$local_cmd; echo \"Fuzzer exit code: \$?\""
else
  printf "> No testcase provided. Binaries synced to %s.\n" "$REMOTE_DIR"
fi

