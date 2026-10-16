# Copyright (C) 2025 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# TODO(jahdiel): Update the file structure such that
#                we use absolute paths and the path looks
#                something like torq.torq
from src import torq

def print_deprecation_warning():
    RED = "\033[1;31m"
    RESET = "\033[0m"

    header = " DEPRECATION WARNING "

    lines = [
        "This version of torq is DEPRECATED.",
        "Torq's development has moved to https://github.com/google/torq",
        "",
        "This version of torq is obsolete and the source code will be deleted soon. Please use",
        "the new GitHub repository to build torq, it has many new features and bug fixes which",
        "aren't available in this version."
    ]

    content_width = max(len(line) for line in lines)

    box_width = max(content_width, len(header))

    total_dash_space = box_width + 2 - len(header)
    left_dashes = "-" * (total_dash_space // 2)
    right_dashes = "-" * (total_dash_space - len(left_dashes))

    print(f"{RED}")
    print(f"+{left_dashes}{header}{right_dashes}+")

    for line in lines:
        print(f"| {line.ljust(box_width)} |")

    print("+" + "-" * (box_width + 2) + "+")
    print(f"{RESET}")

if __name__ == "__main__":
  print_deprecation_warning()
  torq.run()
