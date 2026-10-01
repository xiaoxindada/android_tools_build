Userdata Mount Options
======================

This document contains notes specific to mounting /data in AOSP.

Extra F2FS Devices
------------------

F2FS supports a "devices" option in the "fs\_mgr\_flags" column of its fstab entry. The devices
option attaches extra partitions when `/data` is mounted, and is currently used for Zoned UFS and
landing factory preset data on devices.

### Zoned UFS

AOSP supports [Zoned UFS](https://zonedstorage.io/), with slight modifications needed to fstab. The
"fs\_mgr\_flags" column (the last column) should contain `device=zoned:/dev/block/by-name/zoned_device`.

ueventd automatically creates `/dev/block/by-name/zoned_device` if it detects a partition where
`/sys/class/block/X/queue/zoned` exists and contains something other than `none`. Currently, there
can only be one partition with zones enabled.

A sample fstab entry can be seen
[here](https://cs.android.com/android/platform/superproject/main/+/main:device/google/zumapro/conf/f2fs/fstab.rw.zumapro.f2fs).

The source partition will contain the superblock and must be conventional/non-zoned.

### OOBE Assets

Normally OOBE assets are deployed via the factory ROM or downloaded on demand. For some assets, the
factory ROM might not make sense. This is true for assets that are particularly large, or not OTA
updatable, or are only used for factory testing.

Historically these assets would be bundled into a pre-formatted userdata which would be flashed in
the factory. However this practice is long deprecated and no longer supported, and is difficult to
maintain if the same ROM is used across multiple SKUs.

It is also incompatible with ZUFS, in which case properly formatting `/data` cannot be done by the
host build system.

The new mechanism to do this is by using the "devices" option in the fstab entry. The option should
look like:

    device=exp_alias:/dev/block/by-name/userdata_exp.assets

Here, "exp\_alias" means "expansion partition with an alias file". The alias file is described
further on. `userdata_exp.assets` must be a physical partition. The partition can be flashed with
anything in the factory. You can have up to 7 of these expansion partitions. The name must start
with `userdata_exp.` to automatically pick up the correct SELinux policies.

When the device boots for the first time, F2FS will populate special alias files on `/data` for
each "exp\_alias" device entry. This file tells F2FS that the physical partition is currently
in-use and its blocks should not be modified. If the file is deleted, F2FS will then treat
`userdata_exp.assets` as free space to allocate normal user storage.

The alias file must eventually be deleted to reclaim space. The recommended way to delete the file
is to trigger an init action with the following rules:

    setprop userdata.alias.remove userdata_exp.assets
    start userdata_alias_remove

This will start a system service that unlinks the file and reclaims the space.

Expansion partitions are erased on factory reset. However, the alias file is re-created after
reformatting. Thus it must be deleted again even after factory reset, otherwise, the user will lose
access to that storage space.

Note that expansion partitions appear encrypted when accessed through their alias files. Thus, the
underlying data must be accessed through a raw block device, otherwise it will appear as garbage
(since the device encryption key is not available in the factory). It is important not to store PII
this way; the data should be considered immutable until reclaimed.
