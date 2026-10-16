#!/usr/bin/env python3
# -*- coding: utf-8 -*-
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

"""payload_info: Show information about an update payload."""

from __future__ import absolute_import
from __future__ import print_function

import argparse
import sys
import textwrap
import json

from six.moves import range
try:
  import update_metadata_pb2
except ImportError:
  print("Error: Could not import update_metadata_pb2.", file=sys.stderr)
  print("If you are in an AOSP environment, please build this tool with:", file=sys.stderr)
  print("  m payload_info", file=sys.stderr)
  print("And run the generated binary", file=sys.stderr)
  print("or append system/update_engine/scripts to $PYTHONPATH.", file=sys.stderr)
  raise

import update_payload


MAJOR_PAYLOAD_VERSION_BRILLO = 2

def DisplayValue(key, value):
  """Print out a key, value pair with values left-aligned."""
  if value is not None:
    print('%-*s %s' % (28, key + ':', value))
  else:
    raise ValueError('Cannot display an empty value.')


def DisplayHexData(data, indent=0):
  """Print out binary data as a hex values."""
  for off in range(0, len(data), 16):
    chunk = bytearray(data[off:off + 16])
    print(' ' * indent +
          ' '.join('%.2x' % c for c in chunk) +
          '   ' * (16 - len(chunk)) +
          ' | ' +
          ''.join(chr(c) if 32 <= c < 127 else '.' for c in chunk))


class PayloadCommand:
  """Show basic information about an update payload.

  This command parses an update payload and displays information from
  its header and manifest.
  """

  def __init__(self, options):
    self.options = options
    self.payload = None

  def _DisplayHeader(self):
    """Show information from the payload header."""
    header = self.payload.header
    DisplayValue('Payload version', header.version)
    DisplayValue('Manifest length', header.manifest_len)

  def _DisplayManifest(self):
    """Show information from the payload manifest."""
    manifest = self.payload.manifest
    # pylint: disable=no-member
    DisplayValue('Number of partitions', len(manifest.partitions))
    DisplayValue('Block size', manifest.block_size)
    DisplayValue('Minor version', manifest.minor_version)

    for i, partition in enumerate(manifest.partitions):
      print('\nPartition %d: %s' % (i, partition.partition_name))
      DisplayValue('  Number of ops', len(partition.operations))
      DisplayValue("  Version", partition.version)
      DisplayValue("  COW Size estimate", partition.estimate_cow_size)

      if partition.HasField('hash_tree_data_extent'):
        extent = partition.hash_tree_data_extent
        DisplayValue('  Hash tree data extent', '(%s, %s)' %
                     (extent.start_block, extent.num_blocks))
      if partition.HasField('hash_tree_extent'):
        extent = partition.hash_tree_extent
        DisplayValue('  Hash tree extent', '(%s, %s)' %
                     (extent.start_block, extent.num_blocks))
      if partition.HasField('hash_tree_algorithm'):
        DisplayValue('  Hash tree algorithm', partition.hash_tree_algorithm)
      if partition.HasField('hash_tree_salt'):
        DisplayValue('  Hash tree salt (hex)', partition.hash_tree_salt.hex())

      if partition.HasField('fec_data_extent'):
        extent = partition.fec_data_extent
        DisplayValue('  FEC data extent', '(%s, %s)' %
                     (extent.start_block, extent.num_blocks))
      if partition.HasField('fec_extent'):
        extent = partition.fec_extent
        DisplayValue('  FEC extent', '(%s, %s)' % (extent.start_block,
                                                   extent.num_blocks))
      if partition.HasField('fec_roots'):
        DisplayValue('  FEC roots', partition.fec_roots)
      DisplayValue('  FEC computation on device',
                   partition.HasField('fec_data_extent'))

    if manifest.HasField('dynamic_partition_metadata'):
      print('\nDynamic Partition Metadata:')
      dpm = manifest.dynamic_partition_metadata
      DisplayValue('  Snapshot enabled', dpm.snapshot_enabled)
      DisplayValue('  VABC enabled', dpm.vabc_enabled)
      if dpm.HasField('vabc_compression_param'):
        DisplayValue('  VABC compression param', dpm.vabc_compression_param)
      if dpm.HasField('cow_version'):
        DisplayValue('  COW version', dpm.cow_version)
      if dpm.HasField('compression_factor'):
        DisplayValue('  Compression Factor', dpm.compression_factor)
      if dpm.HasField('disable_ublk'):
        DisplayValue('  Disable UBLK', dpm.disable_ublk)

  def _DisplaySignatures(self):
    """Show information about the signatures from the manifest."""
    header = self.payload.header
    if header.metadata_signature_len:
      offset = header.size + header.manifest_len
      DisplayValue('Metadata signatures blob',
                   'file_offset=%d (%d bytes)' %
                   (offset, header.metadata_signature_len))
      # pylint: disable=invalid-unary-operand-type
      signatures_blob = self.payload.ReadDataBlob(
          -header.metadata_signature_len,
          header.metadata_signature_len)
      self._DisplaySignaturesBlob('Metadata', signatures_blob)
    else:
      print('No metadata signatures stored in the payload')

    manifest = self.payload.manifest
    if manifest.HasField('signatures_offset'):
      # pylint: disable=no-member
      signature_msg = 'blob_offset=%d' % manifest.signatures_offset
      if manifest.signatures_size:
        signature_msg += ' (%d bytes)' % manifest.signatures_size
      DisplayValue('Payload signatures blob', signature_msg)
      signatures_blob = self.payload.ReadDataBlob(manifest.signatures_offset,
                                                  manifest.signatures_size)
      self._DisplaySignaturesBlob('Payload', signatures_blob)
    else:
      print('No payload signatures stored in the payload')

  @staticmethod
  def _DisplaySignaturesBlob(signature_name, signatures_blob):
    """Show information about the signatures blob."""
    signatures = update_metadata_pb2.Signatures()
    signatures.ParseFromString(signatures_blob)
    # pylint: disable=no-member
    print('%s signatures: (%d entries)' %
          (signature_name, len(signatures.signatures)))
    for signature in signatures.signatures:
      print('  version=%s, hex_data: (%d bytes)' %
            (signature.version if signature.HasField('version') else None,
             len(signature.data)))
      DisplayHexData(signature.data, indent=4)


  def _DisplayOps(self, name, operations):
    """Show information about the install operations from the manifest.

    The list shown includes operation type, data offset, data length, source
    extents, source length, destination extents, and destinations length.

    Args:
      name: The name you want displayed above the operation table.
      operations: The operations object that you want to display information
                  about.
    """
    def _DisplayExtents(extents, name):
      """Show information about extents."""
      num_blocks = sum([ext.num_blocks for ext in extents])
      ext_str = ' '.join(
          '(%s,%s)' % (ext.start_block, ext.num_blocks) for ext in extents)
      # Make extent list wrap around at 80 chars.
      ext_str = '\n      '.join(textwrap.wrap(ext_str, 74))
      extent_plural = 's' if len(extents) > 1 else ''
      block_plural = 's' if num_blocks > 1 else ''
      print('    %s: %d extent%s (%d block%s)' %
            (name, len(extents), extent_plural, num_blocks, block_plural))
      print('      %s' % ext_str)

    op_dict = update_payload.common.OpType.NAMES
    print('%s:' % name)
    for op_count, op in enumerate(operations):
      print('  %d: %s' % (op_count, op_dict[op.type]))
      if op.HasField('data_offset'):
        print('    Data offset: %s' % op.data_offset)
      if op.HasField('data_length'):
        print('    Data length: %s' % op.data_length)
      if op.src_extents:
        _DisplayExtents(op.src_extents, 'Source')
      if op.dst_extents:
        _DisplayExtents(op.dst_extents, 'Destination')

  def _GetStats(self, manifest):
    """Returns various statistics about a payload file.

    Returns a dictionary containing the number of blocks read during payload
    application, the number of blocks written, and the number of seeks done
    when writing during operation application.
    """
    read_blocks = 0
    written_blocks = 0
    num_write_seeks = 0
    for partition in manifest.partitions:
      last_ext = None
      for curr_op in partition.operations:
        read_blocks += sum([ext.num_blocks for ext in curr_op.src_extents])
        written_blocks += sum([ext.num_blocks for ext in curr_op.dst_extents])
        for curr_ext in curr_op.dst_extents:
          # See if the extent is contiguous with the last extent seen.
          if last_ext and (curr_ext.start_block !=
                           last_ext.start_block + last_ext.num_blocks):
            num_write_seeks += 1
          last_ext = curr_ext

      # Old and new partitions are read once during verification.
      read_blocks += partition.old_partition_info.size // manifest.block_size
      read_blocks += partition.new_partition_info.size // manifest.block_size

    stats = {'read_blocks': read_blocks,
             'written_blocks': written_blocks,
             'num_write_seeks': num_write_seeks}
    return stats

  def _DisplayStats(self, manifest):
    stats = self._GetStats(manifest)
    DisplayValue('Blocks read', stats['read_blocks'])
    DisplayValue('Blocks written', stats['written_blocks'])
    DisplayValue('Seeks when writing', stats['num_write_seeks'])

  def _GetJsonStats(self, manifest):
    """Returns a dictionary with detailed statistics about the payload."""
    op_names = update_payload.common.OpType.NAMES
    json_stats = {
        'partitions': {},
        'block_size': manifest.block_size,
        'minor_version': manifest.minor_version,
    }
    if manifest.HasField('dynamic_partition_metadata'):
      dpm = manifest.dynamic_partition_metadata
      dpm_stats = {
          'snapshot_enabled': dpm.snapshot_enabled,
          'vabc_enabled': dpm.vabc_enabled,
      }
      if dpm.HasField('vabc_compression_param'):
        dpm_stats['vabc_compression_param'] = dpm.vabc_compression_param
      if dpm.HasField('cow_version'):
        dpm_stats['cow_version'] = dpm.cow_version
      if dpm.HasField('compression_factor'):
        dpm_stats['compression_factor'] = dpm.compression_factor
      if dpm.HasField('disable_ublk'):
        dpm_stats['disable_ublk'] = dpm.disable_ublk
      json_stats['dynamic_partition_metadata'] = dpm_stats

    total_read_blocks = 0
    total_written_blocks = 0
    total_write_seeks = 0

    for partition in manifest.partitions:
      partition_stats = {
          'op_stats': {},
          'read_blocks': 0,
          'written_blocks': 0,
          'num_write_seeks': 0,
          'new_partition_size': partition.new_partition_info.size,
          'old_partition_size': partition.old_partition_info.size,
          'version': partition.version,
          'estimate_cow_size': partition.estimate_cow_size,
      }

      if partition.HasField('hash_tree_data_extent'):
        partition_stats['hash_tree_data_extent'] = {
            'start_block': partition.hash_tree_data_extent.start_block,
            'num_blocks': partition.hash_tree_data_extent.num_blocks,
        }
      if partition.HasField('hash_tree_extent'):
        partition_stats['hash_tree_extent'] = {
            'start_block': partition.hash_tree_extent.start_block,
            'num_blocks': partition.hash_tree_extent.num_blocks,
        }
      if partition.HasField('hash_tree_algorithm'):
        partition_stats['hash_tree_algorithm'] = partition.hash_tree_algorithm
      if partition.HasField('hash_tree_salt'):
        partition_stats['hash_tree_salt'] = partition.hash_tree_salt.hex()
      partition_stats['fec_computation_on_device'] = partition.HasField(
          'fec_data_extent')
      if partition.HasField('fec_data_extent'):
        partition_stats['fec_data_extent'] = {
            'start_block': partition.fec_data_extent.start_block,
            'num_blocks': partition.fec_data_extent.num_blocks,
        }
      if partition.HasField('fec_extent'):
        partition_stats['fec_extent'] = {
            'start_block': partition.fec_extent.start_block,
            'num_blocks': partition.fec_extent.num_blocks,
        }
      if partition.HasField('fec_roots'):
        partition_stats['fec_roots'] = partition.fec_roots

      last_ext = None
      for op in partition.operations:
        op_name = op_names[op.type]
        if op_name not in partition_stats['op_stats']:
          partition_stats['op_stats'][op_name] = {'count': 0, 'blocks': 0}

        partition_stats['op_stats'][op_name]['count'] += 1
        written_blocks_op = sum(ext.num_blocks for ext in op.dst_extents)
        partition_stats['op_stats'][op_name]['blocks'] += written_blocks_op

        partition_stats['read_blocks'] += sum(
            ext.num_blocks for ext in op.src_extents)
        partition_stats['written_blocks'] += written_blocks_op

        for curr_ext in op.dst_extents:
          if last_ext and (curr_ext.start_block !=
                           last_ext.start_block + last_ext.num_blocks):
            partition_stats['num_write_seeks'] += 1
          last_ext = curr_ext

      # Add verification reads
      partition_stats['read_blocks'] += (
          partition.old_partition_info.size // manifest.block_size)
      partition_stats['read_blocks'] += (
          partition.new_partition_info.size // manifest.block_size)

      json_stats['partitions'][partition.partition_name] = partition_stats
      total_read_blocks += partition_stats['read_blocks']
      total_written_blocks += partition_stats['written_blocks']
      total_write_seeks += partition_stats['num_write_seeks']

    json_stats['total_read_blocks'] = total_read_blocks
    json_stats['total_written_blocks'] = total_written_blocks
    json_stats['total_write_seeks'] = total_write_seeks
    return json_stats

  def Run(self):
    """Parse the update payload and display information from it."""
    self.payload = update_payload.Payload(self.options.payload_file)
    self.payload.Init()

    if self.options.json:
      stats = self._GetJsonStats(self.payload.manifest)
      print(json.dumps(stats, indent=2))
      return

    self._DisplayHeader()
    self._DisplayManifest()
    if self.options.signatures:
      self._DisplaySignatures()
    if self.options.stats:
      self._DisplayStats(self.payload.manifest)
    if self.options.list_ops:
      print()
      # pylint: disable=no-member
      for partition in self.payload.manifest.partitions:
        self._DisplayOps('%s install operations' % partition.partition_name,
                         partition.operations)


def main():
  parser = argparse.ArgumentParser(
      description='Show information about an update payload.')
  parser.add_argument('payload_file', type=argparse.FileType('rb'),
                      help='The update payload file.')
  parser.add_argument('--list_ops', default=False, action='store_true',
                      help='List the install operations and their extents.')
  parser.add_argument('--stats', default=False, action='store_true',
                      help='Show information about overall input/output.')
  parser.add_argument('--signatures', default=False, action='store_true',
                      help='Show signatures stored in the payload.')
  parser.add_argument('--json', default=False, action='store_true',
                      help='Dump payload info in JSON format.')
  args = parser.parse_args()

  PayloadCommand(args).Run()


if __name__ == '__main__':
  sys.exit(main())
