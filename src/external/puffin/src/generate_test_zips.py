#!/usr/bin/env python3

#
# Copyright (C) 2015 The Android Open Source Project
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
#

import sys
import zipfile


def main():
  if len(sys.argv) != 3:
    print("Usage: generate_test_zips.py <src.zip> <dst.zip>")
    sys.exit(1)

  src_zip_path = sys.argv[1]
  dst_zip_path = sys.argv[2]

  # Generate deterministic sequence data
  # We create two text contents that have repeating structure but are slightly different.
  src_text1_lines = [
    f"Line {i}: This is some deterministic testing data." for i in range(1000)
  ]
  dst_text1_lines = [
    f"Line {i}: This is some deterministic testing data."
    if i % 10 != 0
    else f"Line {i}: This line was MODIFIED for diffing."
    for i in range(1000)
  ]

  src_text2_lines = [
    f"Data row {i}: Value A={i * 12345}, Value B={i * 54321}" for i in range(500)
  ]
  dst_text2_lines = [
    f"Data row {i}: Value A={i * 54321}, Value B={i * 12345}"
    if i % 5 == 0
    else f"Data row {i}: Value A={i * 12345}, Value B={i * 54321}"
    for i in range(500)
  ]

  src_text1 = "\n".join(src_text1_lines)
  dst_text1 = "\n".join(dst_text1_lines)

  src_text2 = "\n".join(src_text2_lines)
  dst_text2 = "\n".join(dst_text2_lines)

  # Use zipfile.ZIP_DEFLATED to ensure it creates deflate streams compatible with puffin
  with zipfile.ZipFile(src_zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("file1.txt", src_text1)
    zf.writestr("file2.txt", src_text2)

  with zipfile.ZipFile(dst_zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("file1.txt", dst_text1)
    zf.writestr("file2.txt", dst_text2)


if __name__ == "__main__":
  main()
