#!/bin/bash

set -x
set -e

REPO_ROOT=$(realpath $(dirname $0)/../../..)
${REPO_ROOT}/development/tools/external_crates/android_cargo.py run --manifest-path ${REPO_ROOT}/development/tools/external_crates/Cargo.toml -- --managed-repo-path=external/rust/android-crates-io preupload-check $*
${REPO_ROOT}/development/tools/external_crates/android_cargo.py run --manifest-path ${REPO_ROOT}/development/tools/external_crates/Cargo.toml -- --managed-repo-path=external/rust/android-crates-io/extra_versions preupload-check $*
