# 3rd party Rust crates from crates.io

This repository contains Rust crates imported from crates.io for use by the Android platform.

The files in this repository are managed by a tool, which you can run from this directory:

```
./crate_tool help
```

Most of the files here should not be edited manually, and pre-upload checks will catch and prevent such changes.

## How to edit build rules

Do not edit Android.bp files directly. Instead, edit cargo_embargo.json and run `./crate_tool regenerate <crate name>`.

Refer to the [cargo_embargo documentation](https://android.googlesource.com/platform/development/+/main/tools/cargo_embargo/README.md)
for more information.

## How to update a crate

Do not use `external_updater`. Use `crate_tool` as follows:

### Finding updatable crates

This will print every newer version of every crate:

```
./crate_tool updatable-crates
```

### Analyzing updates for a particular crate

To check all newer versions of a crate for potential problems:

```
./crate_tool analyze-updates <crate name>
```

### Updating a crate

To update a crate to a specified version:

```
./crate_tool update <crate name> <version>
```

This does not do any `repo` or `git` operations, so if the update is successful, you will need to run `repo start`, `git commit`, `repo upload`, etc.

Several problems can occur when updating a crate:
* Patches can fail to apply. In this case, you may need to edit or remove the offending patch files in in the `patches/` directory.
* `cargo_embargo` may fail, requiring edits to `cargo_embargo.json`
* Android may fail to build due to missing dependencies or syntax changes in the updated crate.

The update command does not try to build the Android tree, but a recommended local test before `repo upload` is to build everything under `external/rust`:

```
source build/envsetup.sh
lunch aosp_husky-trunk_staging-eng
cd external/rust
mm
```

### Manual updates

Instead of running `./crate_tool update <crate name> <version>`, you can edit `pseudo_crate/Cargo.toml` directly and run
`./crate_tool regenerate <crate name>`. This can be useful when pairs of crates need to be updated in lockstep. This can happen, for example,
when a crate has an associated proc_macro crate. So to update `foo` and `foo_derive` together, edit the versions of both in `pseudo_crate/Cargo.toml`, then run:

```
./crate_tool regenerate foo foo_derive
```

### Keeping crates updated

If you don't have a specific crate you need to update, but want to help keep the crate repository up-to-date, the crate tool can suggest crate updates that seem likely to succeed:

```
./crate_tool suggest-updates
```

## How to add a patch file

You should avoid creating patches, if possible. Every patch file is an ongoing
operational burden that makes it more difficult to keep our crates up-to-date.

If a patch is absolutely necessary, you should, if possible, send a pull
request to the upstream crate, so we can eliminate the Android patch in the
future, when upgrading.

To create a patch for crate "foo", edit the files directly. Then do:

```
git diff --relative=crates/foo -- crates/foo/<file1> crates/foo/<file2> > crates/foo/patches/<name>.patch
```

If you stage or commit the change and the patch, you should see no new changes
when you run "regenerate".

## How to delete a crate

Not a typical operation. Requires a workaround.

* Remove the crate from pseudo_crate/Cargo.toml
* Remove the crate directory `crates/<name>`
* Regenerate any other crate that still exists with `crate_tool regenerate <crate_name>`. This ensures that pseudo_crate/deleted-crates.txt is correct.

## I want to know how the sausage is made

The source code for `crate_tool` is [here](https://android.googlesource.com/platform/development/+/refs/heads/main/tools/external_crates/).

The basic principle of the tool is that every crate directory in
[crates/](https://android.googlesource.com/platform/external/rust/android-crates-io/+/refs/heads/main/crates/)
must be exactly reproducible by an automatic process from a known set of inputs:

* The crate archive from crates.io.
* A limited set of known Android-specific customizations, such as `METADATA`, `TEST_MAPPING`, and `MODULE_LICENSE_*` files.
* Any necessary patches, in the `patches/` directory.
* `cargo_embargo.json`, which is used to generate `Android.bp`.

Therefore, what `./crate_tool regenerate <crate name>` does is:

* Downloads the crate from crates.io, using `cargo vendor`
* Copies (or generates from scratch) `METADATA`, `TEST_MAPPING`, etc.
* Applies patches to the crate.
* Generates `Android.bp` by running cargo_embargo.
* Replaces the crate directory with the regenerated contents.

The pre-upload check does exactly the same thing, but without the final step. Instead of replacing the directory contents, it
checks that what it generated from scratch matches the actual crate directory contents.

Crate update are also built on top of `regenerate`. What `./crate_tool update <crate name> <version>` does is:

* `cargo remove <crate_name>`
* `cargo add <crate_name>@=<version>`
* `./crate_tool regenerate <crate name>`
