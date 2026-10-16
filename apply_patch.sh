#!/bin/bash
LOCALDIR=$(pwd)

function apply() {
	cd $1
	for p in $(find patches -type f -name "*.patch" | LC_ALL=C sort -z); do
		patch --directory $2 --batch -f -N -s -p1 < $p
	done
	cd $LOCALDIR
}

apply $LOCALDIR/patches/logging $LOCALDIR/src/system/logging
apply $LOCALDIR/patches/core $LOCALDIR/src/system/core
apply $LOCALDIR/patches/fs_mgr $LOCALDIR/src/system/fs/fs_mgr
echo "apply patch done!"
