# Simpleperf Python Scripts

This directory contains Python scripts for processing simpleperf data,
generating reports, and converting profiles to formats like pprof.

## Setup

The scripts depend on the Python protobuf library, and assume it's been
installed.

```bash
pip install protobuf
```

Alternatively, you can use a virtual environment and include the necessary paths
from the Android source tree.

```bash
# Create a virtual environment
python3 -m venv venv
source venv/bin/activate

# Add protobuf and simpleperf scripts to PYTHONPATH
export PYTHONPATH=$PYTHONPATH:$(pwd)/external/protobuf/python
export PYTHONPATH=$PYTHONPATH:$(pwd)/system/extras/simpleperf/scripts
```

## Capturing Kernel Symbols

To ensure kernel symbols are captured in your profiles, you must disable certain
security restrictions on the device before running simpleperf:

```bash
adb root
adb shell setprop security.perf_harden 0
adb shell "echo 0 > /proc/sys/kernel/kptr_restrict"
```

## Generating Symbolized pprof Profiles

To generate a pprof profile with symbols from your local build, use the `pprof`
tool and provide the path to unstripped binaries via `PPROF_BINARY_PATH`.

```bash
# Example: Generate a flamegraph URL from perf.data
PPROF_BINARY_PATH=out/target/product/<device>/symbols/system/lib64:out/target/product/<device>/symbols/apex/com.android.runtime/lib64/bionic \
pprof -flame perf.data
```

Note: Ensure that the `perf.data` was collected from a build that matches the
symbols in your `out/` directory (matching Build IDs).

## Automated Symbol Collection

You can use `binary_cache_builder.py` to automatically pull relevant binaries
and `kallsyms` from the device into a local `binary_cache` directory for easier
reporting.

```bash
python3 binary_cache_builder.py -lib out/target/product/<device>/symbols -i perf.data
```
