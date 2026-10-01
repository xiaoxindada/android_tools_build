#!/usr/bin/env python3
#
# Copyright (C) 2026 The Android Open Source Project
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

import argparse
from enum import StrEnum
from html.parser import HTMLParser
import json
from pathlib import Path
import sys
from typing import Any, Callable, List

EventList = List[List[str]]

DEFAULT_EVENT_TABLE_JSON_FILE = Path("event_table.json")
# This is a list of known events that we don't want to add to the
# event_table.json
EVENTS_TO_IGNORE = [
    ["0x001E", "CHAIN", "Chain a pair of event counters"],
]
EVENTS_NUM_TO_IGNORE = {event_num for (event_num, _, _) in EVENTS_TO_IGNORE}


class Architecture(StrEnum):
    """List of all possible values for the "architecture" field."""

    ARM64 = "arm64"


class ArmImplementer(StrEnum):
    """List of all possible values for the "implementer" field."""

    ARM = "0x41"  # Arm Limited.
    BROADCOM = "0x42"  # Broadcom Corporation.
    CAVIUM = "0x43"  # Cavium Inc.
    DEC = "0x44"  # Digital Equipment Corporation.
    FUJITSU = "0x46"  # Fujitsu Ltd.
    INFINEON = "0x49"  # Infineon Technologies AG.
    MOTOROLA_FREESCALE = "0x4D"  # Motorola or Freescale Semiconductor Inc.
    NVIDIA = "0x4E"  # NVIDIA Corporation.
    APPLIED_MICRO = "0x50"  # Applied Micro Circuits Corporation.
    QUALCOMM = "0x51"  # Qualcomm Inc.
    MARVELL = "0x56"  # Marvell International Ltd.
    INTEL = "0x69"  # Intel Corporation.
    AMPERE = "0xC0"  # Ampere Computing.


def parse_arguments() -> argparse.Namespace:
    """Parses the command-line arguments."""
    example_text = """
Example web page to download/save the PMU events:
    https://developer.arm.com/documentation/107771/0103/\
Performance-Monitors-Extension-support/Common-performance-monitoring-unit-events?lang=en

Example command:
    python event_table_json_generator.py C1-Pro 0xD8B \
Arm_C1-Pro_Common_Events.html Arm_C1-Pro_Implementation_Defined_Events.html
"""

    parser = argparse.ArgumentParser(
        description=(
            "Extract PMU events from an HTML file to JSON. You probably "
            "should pipe the output into a text file and then format it. "
        ),
        epilog=example_text,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "cpu_name",
        type=str,
        help="Target CPU Name (e.g., 'C1-Pro')",
    )
    parser.add_argument(
        "partnum",
        type=str,
        help=(
            "Part number for the new CPU (e.g., '0xd4f') for arm64 your can "
            "find the part number in the MIDR-EL1--Main-ID-Register section "
            "of the docs."
        ),
    )
    parser.add_argument(
        "common_events",
        type=validate_file_extension(".html"),
        help=(
            "Path: HTML file containing Extracting Common Events you can "
            "download/save the webpage by going into the browser and saving "
            "the page as a file."
        ),
    )
    parser.add_argument(
        "implementation_defined_events",
        type=validate_file_extension(".html"),
        help=(
            "Path: HTML file containing Extracting Implementation Defined "
            "Events you can download the webpage by going into the browser "
            "and saving the page as a file."
        ),
    )
    parser.add_argument(
        "--event-table-json",
        type=validate_file_extension(".json"),
        default=DEFAULT_EVENT_TABLE_JSON_FILE,
        help="Path to the event_table.json file.",
    )
    parser.add_argument(
        "--implementer",
        type=str,
        default=ArmImplementer.ARM.value,
        choices=[item.value for item in ArmImplementer],
        help=(f"Implementer code from the Arm architecture. "
              f"The default value is {ArmImplementer.ARM.value}"),
    )
    parser.add_argument(
        "--architecture",
        type=str,
        default=Architecture.ARM64.value,
        choices=[item.value for item in Architecture],
        help="Architecture of the target CPU.",
    )

    return parser.parse_args()


def validate_file_extension(extension: str) -> Callable[[str], Path]:
    """Factory for a function to validate file extensions for argparse.

    Args:
      extension: The required file extension (e.g., '.json').

    Returns:
      A function that takes a filename, validates its extension, and returns
      it as a Path object if valid. Raises argparse.ArgumentTypeError
      otherwise.
    """

    def _validate(filename: str) -> Path:
        """The actual validation function."""
        path = Path(filename)
        if path.suffix.lower() != extension:
            raise argparse.ArgumentTypeError(
                f"File '{filename}' must have a '{extension}' extension."
            )
        return path

    return _validate


class _EventTableParser(HTMLParser):
    """A custom HTML parser to extract event table data from an HTML file."""

    def __init__(self) -> None:
        super().__init__()
        self._in_target_table = False
        self._is_parsing_table = False
        self._in_row = False
        self._in_cell = False
        self._in_dt_in_third_cell = False
        self._cell_index = 0
        self._found_dt_in_third_cell = False
        self._current_row: list[str] = []
        self._current_cell_data = ""
        self.extracted_data: list[list[str]] = []

    def handle_starttag(
        self,
        tag: str,
        attrs: list[tuple[str, str | None]],
    ) -> None:
        """Handles the start of a tag.

        Args:
          tag: The name of the tag.
          attrs: A list of tuples containing the tag's attributes.
        """
        if tag == "table" and not self._is_parsing_table:
            self._in_target_table = True
            self._is_parsing_table = True

        if self._in_target_table and tag == "tr":
            self._in_row = True
            self._current_row = []
            self._cell_index = 0
            self._found_dt_in_third_cell = False

        if self._in_row and tag == "td":
            self._in_cell = True
            self._current_cell_data = ""

        if self._in_cell and self._cell_index == 2 and tag == "dt":
            self._in_dt_in_third_cell = True
            self._found_dt_in_third_cell = True
            self._current_cell_data = ""  # Discard previous data in the cell

    def handle_endtag(self, tag: str) -> None:
        """Handles the end of a tag.

        Args:
          tag: The name of the tag.
        """
        if tag == "table" and self._in_target_table:
            self._in_target_table = False

        if tag == "tr" and self._in_row:
            self._in_row = False
            if len(self._current_row) >= 3:
                self.extracted_data.append(self._current_row[:3])

        if tag == "td" and self._in_cell:
            self._in_cell = False
            self._current_row.append(self._current_cell_data.strip())
            self._cell_index += 1

        if tag == "dt" and self._in_dt_in_third_cell:
            self._in_dt_in_third_cell = False

    def handle_data(self, data: str) -> None:
        """Handles the data within a tag.

        Args:
          data: The data to be processed.
        """
        if self._in_cell:
            if self._cell_index == 2:
                if (self._in_dt_in_third_cell or
                        not self._found_dt_in_third_cell):
                    self._current_cell_data += data
            else:
                self._current_cell_data += data


def parse_event_table_local_file(file_path: Path) -> EventList:
    """Parses the event table from a local HTML file using HTMLParser.

    Args:
      file_path: The path to the HTML file containing the event table.

    Returns:
      A list of lists representing the event table data. Each inner list
      contains three elements: event number, mnemonic, and title. Returns an
      empty list if no table is found.
    """
    with open(file_path, "r", encoding="utf-8") as f:
        html_content = f.read()

    parser = _EventTableParser()
    parser.feed(html_content)

    if not parser.extracted_data:
        print(f"Error: No table found in {file_path}", file=sys.stderr)
        return []

    return parser.extracted_data


def parse_event_table_json_file(file_path: Path) -> dict[str, Any]:
    """Parses the event table from a JSON file.

    Args:
      file_path: The path to the JSON file containing the event table.

    Returns:
      A dictionary representing the event table data.
    """
    with open(file_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    return data


def get_new_common_events(
    current_file_events: EventList,
    new_file_events: EventList,
) -> tuple[EventList, EventList]:
    """Returns a list of new common events that are not in the current file.

    Args:
      current_file_events: A list of lists representing the current event table
        data.
      new_file_events: A list of lists representing the new event table data.

    Returns:
      A list of lists representing the new event table data that is not in the
      current file.
    """
    existing_event_nums = {ev_num for (ev_num, _, _) in current_file_events}
    new_events = []
    ignored_events = []
    for event in new_file_events:
        event_num = event[0]
        if event_num not in existing_event_nums:
            if event_num in EVENTS_NUM_TO_IGNORE:
                ignored_events.append(event)
            else:
                new_events.append(event)
    return new_events, ignored_events


def main() -> None:
    args = parse_arguments()

    # Loading the event_table.json file
    event_table_json_file = args.event_table_json
    print(f"Loading event_table.json from {event_table_json_file.absolute()}")
    events_from_file = parse_event_table_json_file(event_table_json_file)

    print("\n--- Parsing Common Events ---")
    common_event_list = parse_event_table_local_file(args.common_events)

    if not common_event_list:
        print("No common events extracted. Exiting.", file=sys.stderr)
        sys.exit(1)

    new_common_events, new_ignored_events = get_new_common_events(
        events_from_file[args.architecture]["events"],
        common_event_list,
    )

    if new_common_events:
        print(
            f"Found {len(new_common_events)} new common events to add to"
            f" '{args.architecture}':"
        )
        for ev_num, mnemonic, title in new_common_events:
            print(f'  ["{ev_num}", "{mnemonic}", "{title}"],')
    else:
        print("No new common events found.")

    if new_ignored_events:
        print(f"\nFound {len(new_ignored_events)} ignored events:")
        for event in new_ignored_events:
            print(event)

    # Filter out the events we don't want to add to common events
    filtered_common_events_nums = [
        event_num
        for (event_num, _, _) in common_event_list
        if event_num not in EVENTS_NUM_TO_IGNORE
    ]

    print("\n--- Parsing Implementation Defined Events  ---")
    new_implementation_events = parse_event_table_local_file(
        args.implementation_defined_events,
    )

    if not new_implementation_events:
        print(
            "No implementation defined events extracted. Exiting.",
            file=sys.stderr,
        )
        sys.exit(1)

    print(f"Found {len(new_implementation_events)} implementation events.")

    print("\n--- New CPU Object ---")
    print("Copy the following object into 'event_table.json' under 'cpus':")
    print(
        json.dumps(
            {
                "name": args.cpu_name,
                "implementer": args.implementer,
                "partnum": args.partnum,
                "common_events": filtered_common_events_nums,
                "implementation_defined_events": new_implementation_events,
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
