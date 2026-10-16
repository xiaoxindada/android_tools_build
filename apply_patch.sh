#!/bin/bash
LOCALDIR=$(pwd)

function apply() {
	for p in $(find $1 -type f -name "*.patch" | LC_ALL=C sort); do
		patch --directory $2 --batch -f -N -s -p1 < $p
	done
}

apply $LOCALDIR/patches/logging $LOCALDIR/src/system/logging
apply $LOCALDIR/patches/core $LOCALDIR/src/system/core
apply $LOCALDIR/patches/fs_mgr $LOCALDIR/src/system/fs/fs_mgr
apply $LOCALDIR/patches/build $LOCALDIR/src/build
echo "apply patch done!"
