# Simpleperf

Android Studio includes a graphical front end to Simpleperf, documented in
[Inspect CPU activity with CPU Profiler](https://developer.android.com/studio/profile/cpu-profiler).
Most users will prefer to use that instead of using Simpleperf directly.

If you prefer to use the command line, Simpleperf is a versatile command-line
CPU profiling tool included in the NDK for Mac, Linux, and Windows.

This file contains documentation for simpleperf maintainers.

There is also [user documentation](doc/README.md).

## Updating the prebuilts in system/extras/simpleperf

Find the latest green build of simpleperf targets in ab/git_main-without-vendor.
Once you have the build id (a 7-digit number) and the build is complete, run the
update script from within the `system/extras/simpleperf` directory:

```sh
$ ./scripts/update.py --build <build id>
```

This will create a new change that you can `repo upload`, then approve and
submit as normal.

For testing, we usually only run python host tests as below:

```sh
$ ./scripts/test/test.py --only-host-test
```

To test all scripts, please use python 3.8+ and install below packages:

```sh
$ pip install bokeh jinja2 pandas protobuf textable
```

## Updating the prebuilts in prebuilts/simpleperf (for NDK)

Find the latest green build of simpleperf targets in ab/git_main-without-vendor.

```sh
$ git clone https://googleplex-android.googlesource.com/platform/prebuilts/simpleperf
$ cd simpleperf
$ ./update --build <build id>
```

Then manually edit `ChangeLog` or use Gemini to generate it.
This will create a new change that we can upload for review.

```sh
$ git push origin HEAD:refs/for/main
```

For testing, we need to test if the scripts run on Darwin/Linux/Windows for different Android
versions in 4 steps:

1. Test on Android emulators running on a Linux x86_64 host, for Android version N to current.

```sh
$ ./test/test.py -d <devices> -r 3
```

The scripts support Android >= N. But it's easier to test old versions on emulators. So we only test
Android N on emulators.

Currently, the tests have problems in clean up. So tests on emulators may fail and take too long to
run. And there are a few known failed cases. Hopefully they will be fixed soon.

2. Test on Android devices connected to a Linux x86_64 host, for Android version O to current.

```sh
$ ./test/test.py -d <devices> -r 3
```

3. Test on an Android device connected to a Darwin x86_64 host, for one of Android version O to
   current.

```sh
$ ./test/test.py -d <devices> -r 1
```

4. Test on an Android device connected to a Windows x86_64 host, for one of Android version O to
   current.

```sh
$ ./test/test.py -d <devices> -r 1
```

To check simpleperf contents released in NDK, we can build the NDK package in an NDK branch.

```sh
$ repo init -u persistent-https://android.git.corp.google.com/platform/manifest -b master-ndk
$ repo sync
$ ndk/checkbuild.py --package --system linux --module simpleperf
```

The NDK package is generated in the `out/` directory.

After merging the prebuilt update, the CL can be cherry-picked to NDK release branches.
