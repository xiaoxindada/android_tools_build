#!/bin/bash
LOCALDIR=$(pwd)

cd $LOCALDIR/src/logging
    git apply $LOCALDIR/patches/logging/*.patch
cd $LOCALDIR
echo "apply patch done!"
