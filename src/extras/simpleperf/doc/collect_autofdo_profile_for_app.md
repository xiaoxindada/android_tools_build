
# Collect AutoFDO profile for NDK apps

[TOC]

## Introduction

Starting with the Pixel 10 25Q4 release, we support collecting AutoFDO profiles for apps on user
builds. The app needs to be debuggable or released with the `<profileable android:shell="true" />`
flag set in its `AndroidManifests.xml`.

## An Example

### 1. Flash a user build on Pixel 10

On Pixel 10, AutoFDO profile collection for user build is supported starting from 25Q4 (BP4A and
later branches).

### 2. Find an app with C/C++ source code

This example uses the EndlessTunnel app from https://github.com/yabinc/simpleperf_demo/tree/master/test_apps/endless-tunnel.


### 3. Add additional debug info to help generating AutoFDO profile

Modify `CMakeLists.txt`.

```cmake

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fdebug-info-for-profiling -mllvm -enable-fs-discriminator=true -mllvm -improved-fs-discriminator=true -funique-internal-linkage-names")

```

endless-tunnel has added the `<profileable android:shell="true" />` flag. So we can build a
released app that is profileable. It's also fine to build a debuggable app.

### 4. Install the app

```sh
$ app/build/outputs/apk/debug$ adb install -t app-release.apk
$ app/build/outputs/apk/debug$ adb shell pm -l | grep tunnel
package:com.google.sample.tunnel
```

### 5. Record AutoFDO profile while running the app

```sh
(host) $ adb shell
mustang:/ $ cd /data/local/tmp
# Run simpleperf record, then start the app, play it for 10 seconds.
mustang:/data/local/tmp $ simpleperf record --app com.google.sample.tunnel -e cs-etm:u --duration 10 -z
simpleperf I environment.cpp:545] Waiting for process of app com.google.sample.tunnel
simpleperf I environment.cpp:537] Got process 23814 for package com.google.sample.tunnel
simpleperf I cmd_record.cpp:826] Recorded for 10.0316 seconds. Start post processing.
simpleperf I cmd_record.cpp:902] Aux data traced: 435,841,357
simpleperf I cmd_record.cpp:894] Record compressed: 15.24 MB (original 417.84 MB, ratio 27)

# Convert perf.data to branch list file.
# This can be done on device, but here we pull the file to the host for conversion.
(host) $ adb pull /data/local/tmp/perf.data
(host) $ simpleperf inject -i perf.data --output branch-list -o branch_list.data
# Dump branch list file, which shows branch lists for libgame.so.
(host) $ simpleperf inject --dump branch_list.data >branch_list.dump
(host) $ cat branch_list.dump | grep path
  ...
  binary[18].path: /data/app/~~CYyLxmCI1QHYKFwuWNvoUA==/com.google.sample.tunnel-c0qGzW5EkVZtHZXqGPZIoQ==/base.apk!/lib/arm64-v8a/libgame.so
  ...
# Convert branch lists for libgame.so to AutoFDO profile.
(host) $ simpleperf inject -i branch_list.data -o libgame_autofdo.txt --binary libgame.so --log debug
simpleperf D command.cpp:288] command 'inject' starts running
simpleperf D cmd_inject.cpp:637] Not found debug binary for /data/app/~~CYyLxmCI1QHYKFwuWNvoUA==/com.google.sample.tunnel-c0qGzW5EkVZtHZXqGPZIoQ==/base.apk!/lib/arm64-v8a/libgame.so with build id 0x288376bbae66d18a0e54e81cbef194b2342aa880
simpleperf D command.cpp:291] command 'inject' finished successfully
# The warning shows simpleperf didn't find debug binary for libgame.so. Let's provide it.
(host) $ $ simpleperf inject -i branch_list.data -o libgame_autofdo.txt --binary libgame.so --log debug --symdir ~/AndroidStudioProjects/simpleperf_demo/test_apps/endless-tunnel/app/build/intermediates/cxx/RelWithDebInfo/5j224z5k/obj/arm64-v8a
simpleperf D command.cpp:288] command 'inject' starts running
simpleperf D command.cpp:291] command 'inject' finished successfully
(host) $ ls -lh
-rw-r--r-- 1 yabinc primarygroup  52K Sep 22 15:39 libgame_autofdo.txt

# Run create_llvm_prof to convert AutoFDO input file to AutoFDO profile.
# create_llvm_prof can be downloaded or built from https://github.com/google/autofdo.
(host) $ create_llvm_prof --profiler text -profile=libgame_autofdo.txt --out=libgame.llvm_profdata --prof_sym_list=false --format=extbinary --binary ~/AndroidStudioProjects/simpleperf_demo/test_apps/endless-tunnel/app/build/intermediates/cxx/RelWithDebInfo/5j224z5k/obj/arm64-v8a/libgame.so
(host) $ ls -lh
-rw-r--r-- 1 yabinc primarygroup  18K Sep 22 15:44 libgame.llvm_profdata

# Show hot functions in libgame.llvm_profdata.
# llvm-profdata can be downloaded or built from https://github.com/llvm/llvm-project.
(host) $ llvm-profdata show --sample --hot-func-list libgame.llvm_profdata >libgame_hot_functions.txt
```

Currently we are recording ETM data for all userspace libraries for the app process. But we only
want to optimize `libgame.so`. ETM data for other libraries are wasting bandwidth. So we can use
address filter to only capture ETM data for `libgame.so`.

```sh
mustang:/data/local/tmp $ simpleperf record --app com.google.sample.tunnel -e cs-etm:u \
  --duration 10 -z --addr-filter 'filter /data/app/~~CYyLxmCI1QHYKFwuWNvoUA==/com.google.sample.tunnel-c0qGzW5EkVZtHZXqGPZIoQ==/base.apk!/lib/arm64-v8a/libgame.so'

# It's also fine to use the whole apk file as the address filter.
mustang:/data/local/tmp $ simpleperf record --app com.google.sample.tunnel -e cs-etm:u \
  --duration 10 -z --addr-filter 'filter /data/app/~~CYyLxmCI1QHYKFwuWNvoUA==/com.google.sample.tunnel-c0qGzW5EkVZtHZXqGPZIoQ==/base.apk'
```

To get good coverage, you usually need to record multiple times and record for a longer duration
each time. Multiple `perf.data` or `branch_list.data` files can be merged into one when converting
them to an AutoFDO profile.

```sh
(host) simpleperf inject -i perf.data,perf2.data,perf3.data,perf4.data,perf5.data -o branch_list.data --output branch-list
(host) $ $ simpleperf inject -i branch_list.data -o libgame_autofdo.txt --binary libgame.so --log debug --symdir ~/AndroidStudioProjects/simpleperf_demo/test_apps/endless-tunnel/app/build/intermediates/cxx/RelWithDebInfo/5j224z5k/obj/arm64-v8a
```

### 6. Use AutoFDO profile when building the app


```sh
# Copy libgame.llvm_profdata to the source code directory.
(host) $ cp libgame.llvm_profdata ~/AndroidStudioProjects/simpleperf_demo/test_apps/endless-tunnel/app/src/main/cpp
```

Modify `CMakeLists.txt`.

```cmake

# Set clang options to use AutoFDO profile
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fdebug-info-for-profiling -mllvm -enable-fs-discriminator=true -mllvm -improved-fs-discriminator=true -funique-internal-linkage-names")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fprofile-sample-use=${CMAKE_CURRENT_SOURCE_DIR}/libgame.llvm_profdata")

# It's optional to add -fprofile-sample-accurate. Once added, it may de-optimize cold functions.
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fprofile-sample-accurate")

# Set dependency on the profile. So the source files are rebuilt when the profile is changed.
set_source_files_properties(${MY_SOURCES} PROPERTIES OBJECT_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/libgame.llvm_profdata)

```

To get performance gains from AutoFDO, you need to enable optimization, which means using `-O2`
or `-O3` in `CMakeLists.txt`, or creating a Release build. We can compare the .text section before
and after using AutoFDO profile. In most cases, it should change the .text of the binary, which can
be quickly verified by checking that the size of the .text changes when the AutoFDO profile is used.

```sh
(host) readelf -SW without_profile_libgame.so
[14] .text             PROGBITS        0000000000027c90 027c90 02d974 00  AX  0   0 16
(host) readelf -SW with_profile_libgame.so
[14] .text             PROGBITS        0000000000028840 028840 02d77c 00  AX  0   0 16
```
