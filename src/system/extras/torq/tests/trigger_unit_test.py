#
# Copyright (C) 2025 The Android Open Source Project
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

import os
import time
import subprocess
import sys
import unittest

from src.base import ANDROID_SDK_VERSION_T
from src.config import create_config_command
from src.device import AdbDevice
from src.profiler import DEFAULT_DUR_MS, DEFAULT_OUT_DIR, PERFETTO_TRACE_FILE
from src.torq import create_parser, verify_args
from tests.test_utils import create_parser_from_cli, parse_cli, run_cli
from unittest import mock
from unittest.mock import ANY

TEST_SERIAL = "test-serial"
TEST_TRIGGER_NAMES = [
    "team.package.test-trigger-name", "team2.package2.test-trigger-name2"
]
TEST_TRIGGER_DUR_MS = 10000
TEST_TRIGGER_STOP_DELAY_MS = 1000
TEST_MULTIPLE_TRIGGER_STOP_DELAY_MS = ["1000", "2000"]
TEST_TRIGGER_MODE = "stop"


class TriggerSubcommandUnitTest(unittest.TestCase):

  @mock.patch('src.torq.AdbDevice', autospec=True)
  def test_trigger_names(self, mock_device):
    self.mock_device = mock_device.return_value
    self.mock_device.check_device_connection.return_value = None

    run_cli("torq trigger %s" % TEST_TRIGGER_NAMES[0])

    self.mock_device.trigger_perfetto.assert_called_with(TEST_TRIGGER_NAMES[0])


class ProfilerTriggerUnitTest(unittest.TestCase):

  def setUp(self):
    self.mock_device = mock.create_autospec(
        AdbDevice, instance=True, serial=TEST_SERIAL)
    self.mock_device.check_device_connection.return_value = None
    self.mock_device.get_android_sdk_version.return_value = (
        ANDROID_SDK_VERSION_T)
    self.mock_device.create_directory.return_value = None
    self.mock_device.remove_file.return_value = False
    self.mock_device.pull_file.return_value = False
    self.mock_sleep_patcher = mock.patch.object(
        time, 'sleep', return_value=None)
    self.mock_sleep_patcher.start()

  def tearDown(self):
    self.mock_sleep_patcher.stop()

  @mock.patch('src.torq.AdbDevice', autospec=True)
  def test_trigger_names(self, mock_device_creator):
    mock_device_creator.return_value = self.mock_device

    with (mock.patch("src.profiler.open_trace", autospec=True) as
          mock_open_trace):
      mock_open_trace.return_value = None
      run_cli("torq --trigger-names %s" % " ".join(TEST_TRIGGER_NAMES))

    mock_device_creator.assert_called_once_with(None)

    self.mock_device.pull_file.assert_called_with(PERFETTO_TRACE_FILE, ANY)

    self.mock_device.start_perfetto_trace.assert_called()

  @mock.patch('src.torq.AdbDevice', autospec=True)
  def test_trigger_names_with_stop_delay(self, mock_device_creator):
    mock_device_creator.return_value = self.mock_device

    with (mock.patch("src.profiler.open_trace", autospec=True) as
          mock_open_trace):
      mock_open_trace.return_value = None
      run_cli("torq --trigger-names %s --trigger-stop-delay-ms %d" %
              (" ".join(TEST_TRIGGER_NAMES), TEST_TRIGGER_STOP_DELAY_MS))

    mock_device_creator.assert_called_once_with(None)

    self.mock_device.pull_file.assert_called_with(PERFETTO_TRACE_FILE, ANY)

    self.mock_device.start_perfetto_trace.assert_called()

  @mock.patch('src.torq.AdbDevice', autospec=True)
  def test_trigger_names_clone_mode(self, mock_device_creator):
    mock_device_creator.return_value = self.mock_device

    with (mock.patch("src.profiler.open_trace", autospec=True) as
          mock_open_trace):
      mock_open_trace.return_value = None
      run_cli("torq --trigger-names %s --trigger-mode %s" %
              (" ".join(TEST_TRIGGER_NAMES), "clone"))

    mock_device_creator.assert_called_once_with(None)

    self.mock_device.pull_file.assert_called_with(PERFETTO_TRACE_FILE + ".0",
                                                  ANY)

    self.mock_device.start_perfetto_trace.assert_called()

  @mock.patch('src.torq.AdbDevice', autospec=True)
  def test_trigger_names_start_mode(self, mock_device_creator):
    mock_device_creator.return_value = self.mock_device

    with (mock.patch("src.profiler.open_trace", autospec=True) as
          mock_open_trace):
      mock_open_trace.return_value = None
      run_cli("torq --trigger-names %s --trigger-mode %s" %
              (" ".join(TEST_TRIGGER_NAMES), "start"))

    mock_device_creator.assert_called_once_with(None)

    self.mock_device.pull_file.assert_called_with(PERFETTO_TRACE_FILE, ANY)

    self.mock_device.start_perfetto_trace.assert_called()

  @mock.patch('src.torq.AdbDevice', autospec=True)
  def test_trigger_names_multiple_stop_delays(self, mock_device_creator):
    mock_device_creator.return_value = self.mock_device

    with (mock.patch("src.profiler.open_trace", autospec=True) as
          mock_open_trace):
      mock_open_trace.return_value = None
      run_cli("torq --trigger-names %s --trigger-stop-delay-ms %s" %
              (" ".join(TEST_TRIGGER_NAMES),
               " ".join(TEST_MULTIPLE_TRIGGER_STOP_DELAY_MS)))

    mock_device_creator.assert_called_once_with(None)

    self.mock_device.pull_file.assert_called_with(PERFETTO_TRACE_FILE, ANY)

    self.mock_device.start_perfetto_trace.assert_called()

  @mock.patch('src.torq.AdbDevice', autospec=True)
  def test_trigger_names_incorrect_stop_delays(self, mock_device_creator):
    mock_device_creator.return_value = self.mock_device

    with (mock.patch("src.profiler.open_trace", autospec=True) as
          mock_open_trace):
      mock_open_trace.return_value = None
      run_cli("torq --trigger-names %s --trigger-stop-delay-ms %s %s" %
              (" ".join(TEST_TRIGGER_NAMES),
               " ".join(TEST_MULTIPLE_TRIGGER_STOP_DELAY_MS), "3000"))

    mock_device_creator.assert_not_called()

    self.mock_device.pull_file.assert_not_called()

    self.mock_device.start_perfetto_trace.assert_not_called()


if __name__ == '__main__':
  unittest.main()
