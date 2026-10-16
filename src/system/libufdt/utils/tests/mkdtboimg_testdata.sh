#! /bin/bash
# Copyright (C) 2017 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
set -e

SRCDIR="data"
DTS_LIST="
  board1v1.dts
  board1v1_1.dts
  board2v1.dts
"
DTB_LIST=(
  "board1v1.dts.dtb"
  "board1v1_1.dts.dtb"
  "board2v1.dts.dtb"
  "board1v1.dts.dtb"
)
PYCONFIG="${SRCDIR}/mkdtboimg.cfg"
PYCONFIG_V2="${SRCDIR}/mkdtboimg_v2.cfg"

ALIGN=4

OUTDIR="out"
MKDTIMG_OUT="${OUTDIR}/mkdtimg_out"
MKDTIMG_DUMP="${MKDTIMG_OUT}"/dump.dtb

MKDTBOIMG_OUT="${OUTDIR}/mkdtboimg_out"
MKDTBOIMG_OUTCREATE="${MKDTBOIMG_OUT}/create"
MKDTBOIMG_OUTCREATE_V2="${MKDTBOIMG_OUT}/create_v2"
MKDTBOIMG_OUTCFG="${MKDTBOIMG_OUT}/cfg_create"
MKDTBOIMG_OUTCFG_V2="${MKDTBOIMG_OUT}/cfg_create_v2"
MKDTBOIMG_CREATEDUMP="${MKDTBOIMG_OUTCREATE}"/dump.dtb
MKDTBOIMG_CREATEDUMP_V2="${MKDTBOIMG_OUTCREATE_V2}"/dump.dtb
MKDTBOIMG_CFGDUMP="${MKDTBOIMG_OUTCFG}"/dump.dtb
MKDTBOIMG_CFGDUMP_V2="${MKDTBOIMG_OUTCFG_V2}"/dump.dtb

mkdir -p "$MKDTIMG_OUT"
mkdir -p "$MKDTBOIMG_OUTCREATE"
mkdir -p "$MKDTBOIMG_OUTCREATE_V2"
mkdir -p "$MKDTBOIMG_OUTCFG"
mkdir -p "$MKDTBOIMG_OUTCFG_V2"

for dts in ${DTS_LIST}; do
  echo "Building $dts..."
  src_dts="${SRCDIR}/${dts}"
  out_dtb="${OUTDIR}/${dts}.dtb"
  dtc -O dtb -@ -qq -a "$ALIGN" -o "$out_dtb" "$src_dts"
done

echo "Creating dtbo image with mkdtbimg"
mkdtimg create ${MKDTIMG_OUT}/create.img --page_size=4096 --id=0x100 --version=1\
    --rev=0x100 --custom0=0xabc "${OUTDIR}/board1v1.dts.dtb" "${OUTDIR}/board1v1_1.dts.dtb" \
    --id=0xddccbbaa --rev=0x01000100 "${OUTDIR}/board2v1.dts.dtb" --id=0x200 \
    --rev=0x201 "${OUTDIR}/board1v1.dts.dtb" --custom0=0xdef \
    "${OUTDIR}/board1v1.dts.dtb" --custom0=0xdef > /dev/null

echo "Creating dtbo image with mkdtboimg"
../src/mkdtboimg.py create  ${MKDTBOIMG_OUTCREATE}/create.img --page_size=4096 \
    --id=0x100 --rev=0x100 --flags=0xabc0 --version=1 --custom0=0xabc \
    "${OUTDIR}/board1v1.dts.dtb" \
    "${OUTDIR}/board1v1_1.dts.dtb" --id=0xddccbbaa --rev=0x01000100 \
    "${OUTDIR}/board2v1.dts.dtb" --id=0x200 --rev=0x201 \
    "${OUTDIR}/board1v1.dts.dtb" --flags=0xd01 --custom0=0xdef \
    "${OUTDIR}/board1v1.dts.dtb" --flags=0xd02 --custom0=3567 \
    "${OUTDIR}/board1v1.dts.dtb" --flags=0xd03 > /dev/null

echo "Creating dtbo image with mkdtboimg (v2)"
../src/mkdtboimg.py create ${MKDTBOIMG_OUTCREATE_V2}/create.img --page_size=4096 \
    --id=0x1234 --rev=0x5678 --flags=0xabc0 --version=2 \
    --custom0=0x0 --custom1=0x1 --custom2=0x2 --custom3=0x3 \
    --custom4=0x4 --custom5=0x5 --custom6=0x6 --custom7=0x7 \
    --custom8=0x8 --custom9=0x9 --custom10=0xa \
    "${OUTDIR}/board1v1.dts.dtb" \
    "${OUTDIR}/board2v1.dts.dtb" --id=0x4321 --rev=0x8765 --custom10=0xffffffff > /dev/null

echo "Creating dtbo image with ${PYCONFIG} config file"
../src/mkdtboimg.py cfg_create ${MKDTBOIMG_OUTCFG}/create.img ${PYCONFIG} --dtb-dir "${OUTDIR}"

echo "Creating dtbo image with ${PYCONFIG_V2} config file"
../src/mkdtboimg.py cfg_create ${MKDTBOIMG_OUTCFG_V2}/create.img ${PYCONFIG_V2} --dtb-dir "${OUTDIR}"

echo "Dumping fragments from mkdtimg tool image"
mkdtimg dump ${MKDTIMG_OUT}/create.img -b "${MKDTIMG_DUMP}"| grep -v 'FDT' > ${MKDTIMG_OUT}/create.dump

echo "Dumping fragments from mkdtboimg.py tool for image generated with 'create'"
../src/mkdtboimg.py dump ${MKDTBOIMG_OUTCREATE}/create.img --output ${MKDTBOIMG_OUTCREATE}/create.dump -b "${MKDTBOIMG_CREATEDUMP}" --decompress

echo "Dumping fragments from mkdtboimg.py tool for image generated with 'cfg_create'"
../src/mkdtboimg.py dump ${MKDTBOIMG_OUTCFG}/create.img --output ${MKDTBOIMG_OUTCFG}/create.dump -b "${MKDTBOIMG_CFGDUMP}" --decompress

echo "Dumping fragments from mkdtboimg.py tool for image generated with 'create' (v2)"
../src/mkdtboimg.py dump ${MKDTBOIMG_OUTCREATE_V2}/create.img --output ${MKDTBOIMG_OUTCREATE_V2}/create.dump -b "${MKDTBOIMG_CREATEDUMP_V2}" --decompress

echo "Dumping fragments from mkdtboimg.py tool for image generated with 'cfg_create' (v2)"
../src/mkdtboimg.py dump ${MKDTBOIMG_OUTCFG_V2}/create.img --output ${MKDTBOIMG_OUTCFG_V2}/create.dump -b "${MKDTBOIMG_CFGDUMP_V2}" --decompress

echo "======================================================================================"
echo "Testing differences between image created by 'create' for 'mkdtimg' and 'mkdtboimg.py'"
echo "======================================================================================"
for x in `ls -1 ${MKDTIMG_DUMP}.*`
do
    file=`basename $x`
    if [ ! -e ${MKDTBOIMG_OUTCREATE}/$file ]
    then
        continue
    fi
    echo "diff $x vs ${MKDTBOIMG_OUTCREATE}/$file"
    diff $x ${MKDTBOIMG_OUTCREATE}/$file
done
echo "=========================================================================================="
echo "Testing differences between image created by 'cfg_create' for 'mkdtimg' and 'mkdtboimg.py'"
echo "=========================================================================================="
for x in `ls -1 ${MKDTIMG_DUMP}.*`
do
    file=`basename $x`
    if [ ! -e ${MKDTBOIMG_OUTCFG}/$file ]
    then
        continue
    fi
    echo "diff $x vs ${MKDTBOIMG_OUTCFG}/$file"
    diff $x ${MKDTBOIMG_OUTCFG}/$file
done
echo "=========================================================================================="
echo "Testing image size created by 'create' 'mkdtboimg.py'"
echo "=========================================================================================="
create_img_size=$(stat -c %s "${MKDTBOIMG_OUTCREATE}/create.img")
if (( create_img_size % 4096 == 0 )); then
    echo "Size of ${MKDTBOIMG_OUTCREATE}/create.img is aligned to page_size 4096"
else
    echo "ERROR: Size of ${MKDTBOIMG_OUTCREATE}/create.img is not aligned to page_size 4096"
fi
echo "=========================================================================================="
echo "Testing image size created by 'cfg_create' 'mkdtboimg.py'"
echo "=========================================================================================="
cfg_img_size=$(stat -c %s "${MKDTBOIMG_OUTCFG}/create.img")
if (( cfg_img_size % 4096 == 0 )); then
    echo "Size of ${MKDTBOIMG_OUTCFG}/create.img is aligned to page_size 4096"
else
    echo "ERROR: Size of ${MKDTBOIMG_OUTCFG}/create.img is not aligned to page_size 4096"
fi
echo "=========================================================================================="
echo "Testing image size created by 'create' (v2) 'mkdtboimg.py'"
echo "=========================================================================================="
create_v2_img_size=$(stat -c %s "${MKDTBOIMG_OUTCREATE_V2}/create.img")
if (( create_v2_img_size % 4096 == 0 )); then
    echo "Size of ${MKDTBOIMG_OUTCREATE_V2}/create.img is aligned to page_size 4096"
else
    echo "ERROR: Size of ${MKDTBOIMG_OUTCREATE_V2}/create.img is not aligned to page_size 4096"
fi
echo "=========================================================================================="
echo "Testing image size created by 'cfg_create' (v2) 'mkdtboimg.py'"
echo "=========================================================================================="
cfg_v2_img_size=$(stat -c %s "${MKDTBOIMG_OUTCFG_V2}/create.img")
if (( cfg_v2_img_size % 4096 == 0 )); then
    echo "Size of ${MKDTBOIMG_OUTCFG_V2}/create.img is aligned to page_size 4096"
else
    echo "ERROR: Size of ${MKDTBOIMG_OUTCFG_V2}/create.img is not aligned to page_size 4096"
fi
