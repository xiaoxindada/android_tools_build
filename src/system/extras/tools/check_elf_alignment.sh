#!/bin/bash
# This script is a wrapper around aac.py for ELF alignment checking.
# The real tool is at system/memory/libmeminfo/libelfutils/tools/aac/aac.py

REAL_PATH=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$REAL_PATH")

# Expected relative path in the Android source tree:
# system/extras/tools/ -> system/memory/libmeminfo/libelfutils/tools/aac/aac.py
AAC_PY="$SCRIPT_DIR/../../memory/libmeminfo/libelfutils/tools/aac/aac.py"

if [ ! -f "$AAC_PY" ]; then
    echo "
Error: check_elf_alignment.sh is now a wrapper around aac.py.

The real tool was expected at:
    $AAC_PY

This script seems to have been moved or the Android source tree
structure has changed.

Please run it from its original location in the Android source tree:
    system/extras/tools/check_elf_alignment.sh
" >&2
    exit 1
fi

exec "$AAC_PY" "$@"
