#!/bin/bash

set -euxo pipefail

cd "$(dirname "$(realpath "$0")")"

export RUST_TEST_THREADS=1

rep=$(seq 1 3)

for _ in $rep; do
    cargo nextest run --all-features --nocapture
done

export RUSTFLAGS='-Zsanitizer=address'
export RUSTDOCFLAGS="$RUSTFLAGS"
for _ in $rep; do
    cargo +nightly nextest run --all-features --nocapture
done

#export MIRIFLAGS="-Zmiri-disable-isolation"
exec cargo +nightly miri nextest run --all-features --nocapture
