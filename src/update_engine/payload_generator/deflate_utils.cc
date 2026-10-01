//
// Copyright (C) 2017 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#include "update_engine/payload_generator/deflate_utils.h"

#include <algorithm>
#include <string>
#include <utility>
#include <vector>

#include <base/files/file_util.h>
#include <base/logging.h>

#include "update_engine/common/utils.h"
#include "update_engine/payload_generator/delta_diff_generator.h"
#include "update_engine/payload_generator/extent_ranges.h"
#include "update_engine/payload_generator/extent_utils.h"
#include "update_engine/update_metadata.pb.h"

using puffin::BitExtent;
using puffin::ByteExtent;
using std::string;
using std::vector;

namespace chromeos_update_engine {
namespace deflate_utils {
namespace {

constexpr std::ostream& operator<<(std::ostream& out,
                                   const puffin::BitExtent& ext) {
  out << "BitExtent(" << ext.offset << "," << ext.length << ")";
  return out;
}

bool IsRegularFile(const FilesystemInterface::File& file) {
  // If inode is 0, then stat information is invalid for some psuedo files
  if (file.file_stat.st_ino != 0 &&
      (file.file_stat.st_mode & S_IFMT) == S_IFREG) {
    return true;
  }
  return false;
}

bool IsBitExtentInExtent(const Extent& extent, const BitExtent& bit_extent) {
  return (bit_extent.offset / 8) >= (extent.start_block() * kBlockSize) &&
         ((bit_extent.offset + bit_extent.length + 7) / 8) <=
             ((extent.start_block() + extent.num_blocks()) * kBlockSize);
}
}  // namespace

// Returns whether the given file |name| has an extension listed in
// |extensions|.
constexpr base::StringPiece ToStringPiece(std::string_view s) {
  return base::StringPiece(s.data(), s.length());
}

bool IsFileExtensions(
    const std::string_view name,
    const std::initializer_list<std::string_view>& extensions) {
  return any_of(extensions.begin(),
                extensions.end(),
                [name = ToStringPiece(name)](const auto& ext) {
                  return android::base::EndsWith(ToLower(name), ToLower(ext));
                });
}

ByteExtent ExpandToByteExtent(const BitExtent& extent) {
  uint64_t offset = extent.offset / 8;
  uint64_t length = ((extent.offset + extent.length + 7) / 8) - offset;
  return {offset, length};
}

bool ShiftExtentsOverExtents(const vector<Extent>& base_extents,
                             vector<Extent>* over_extents) {
  if (utils::BlocksInExtents(base_extents) <
      utils::BlocksInExtents(*over_extents)) {
    LOG(ERROR) << "over_extents have more blocks than base_extents! Invalid!";
    return false;
  }
  for (size_t idx = 0; idx < over_extents->size(); idx++) {
    auto over_ext = &over_extents->at(idx);
    auto gap_blocks = base_extents[0].start_block();
    auto last_end_block = base_extents[0].start_block();
    for (auto base_ext : base_extents) {  // We need to modify |base_ext|, so we
                                          // use copy.
      gap_blocks += base_ext.start_block() - last_end_block;
      last_end_block = base_ext.start_block() + base_ext.num_blocks();
      base_ext.set_start_block(base_ext.start_block() - gap_blocks);
      if (over_ext->start_block() >= base_ext.start_block() &&
          over_ext->start_block() <
              base_ext.start_block() + base_ext.num_blocks()) {
        if (over_ext->start_block() + over_ext->num_blocks() <=
            base_ext.start_block() + base_ext.num_blocks()) {
          // |over_ext| is inside |base_ext|, increase its start block.
          over_ext->set_start_block(over_ext->start_block() + gap_blocks);
        } else {
          // |over_ext| spills over this |base_ext|, split it into two.
          auto new_blocks = base_ext.start_block() + base_ext.num_blocks() -
                            over_ext->start_block();
          vector<Extent> new_extents = {
              ExtentForRange(gap_blocks + over_ext->start_block(), new_blocks),
              ExtentForRange(over_ext->start_block() + new_blocks,
                             over_ext->num_blocks() - new_blocks)};
          *over_ext = new_extents[0];
          over_extents->insert(std::next(over_extents->begin(), idx + 1),
                               new_extents[1]);
        }
        break;  // We processed |over_ext|, so break the loop;
      }
    }
  }
  return true;
}

bool ShiftBitExtentsOverExtents(const vector<Extent>& base_extents,
                                vector<BitExtent>* over_extents) {
  if (over_extents->empty()) {
    return true;
  }

  // This check is needed to make sure the number of bytes in |over_extents|
  // does not exceed |base_extents|.
  auto last_extent = ExpandToByteExtent(over_extents->back());
  TEST_LE(last_extent.offset + last_extent.length,
          utils::BlocksInExtents(base_extents) * kBlockSize);

  for (auto o_ext = over_extents->begin(); o_ext != over_extents->end();) {
    size_t gap_blocks = base_extents[0].start_block();
    size_t last_end_block = base_extents[0].start_block();
    bool o_ext_processed = false;
    for (auto b_ext : base_extents) {  // We need to modify |b_ext|, so we copy.
      gap_blocks += b_ext.start_block() - last_end_block;
      last_end_block = b_ext.start_block() + b_ext.num_blocks();
      b_ext.set_start_block(b_ext.start_block() - gap_blocks);
      auto byte_o_ext = ExpandToByteExtent(*o_ext);
      if (byte_o_ext.offset >= b_ext.start_block() * kBlockSize &&
          byte_o_ext.offset <
              (b_ext.start_block() + b_ext.num_blocks()) * kBlockSize) {
        if ((byte_o_ext.offset + byte_o_ext.length) <=
            (b_ext.start_block() + b_ext.num_blocks()) * kBlockSize) {
          // |o_ext| is inside |b_ext|, increase its start block.
          o_ext->offset += gap_blocks * kBlockSize * 8;
          ++o_ext;
        } else {
          // |o_ext| spills over this |b_ext|, remove it.
          o_ext = over_extents->erase(o_ext);
        }
        o_ext_processed = true;
        break;  // We processed o_ext, so break the loop;
      }
    }
    TEST_AND_RETURN_FALSE(o_ext_processed);
  }
  return true;
}

vector<BitExtent> FindDeflates(const vector<Extent>& extents,
                               const vector<BitExtent>& in_deflates) {
  vector<BitExtent> result;
  // TODO(ahassani): Replace this with binary_search style search.
  for (const auto& deflate : in_deflates) {
    for (const auto& extent : extents) {
      if (IsBitExtentInExtent(extent, deflate)) {
        result.push_back(deflate);
        break;
      }
    }
  }
  return result;
}

bool CompactDeflates(const vector<Extent>& extents,
                     const vector<BitExtent>& in_deflates,
                     vector<BitExtent>* out_deflates) {
  size_t bytes_passed = 0;

  out_deflates->reserve(in_deflates.size());

  std::vector<bool> bitmask(in_deflates.size(), false);
  for (const auto& extent : extents) {
    size_t gap_bytes = extent.start_block() * kBlockSize - bytes_passed;
    for (size_t i = 0; i < in_deflates.size(); i++) {
      if (bitmask[i]) {
        continue;
      }
      auto deflate = in_deflates[i];
      if (IsBitExtentInExtent(extent, deflate)) {
        out_deflates->emplace_back(deflate.offset - (gap_bytes * 8),
                                   deflate.length);
        bitmask[i] = true;
      }
    }
    bytes_passed += extent.num_blocks() * kBlockSize;
  }

  // All given |in_deflates| items should've been inside one of the extents in
  // |extents|.
  TEST_EQ(in_deflates.size(), out_deflates->size());
  Dedup(out_deflates);

  // Make sure all outgoing deflates are ordered and non-overlapping.
  auto result = std::adjacent_find(out_deflates->begin(),
                                   out_deflates->end(),
                                   [](const BitExtent& a, const BitExtent& b) {
                                     return (a.offset + a.length) > b.offset;
                                   });
  if (result != out_deflates->end()) {
    LOG(ERROR) << "out_deflate is overlapped " << (*result) << ", "
               << *(++result);
    return false;
  }
  return true;
}

bool FindAndCompactDeflates(const vector<Extent>& extents,
                            const vector<BitExtent>& in_deflates,
                            vector<BitExtent>* out_deflates) {
  auto found_deflates = FindDeflates(extents, in_deflates);
  TEST_AND_RETURN_FALSE(CompactDeflates(extents, found_deflates, out_deflates));
  return true;
}

bool DeflatePreprocessFileData(const std::string_view filename,
                               const brillo::Blob& data,
                               vector<puffin::BitExtent>* deflates) {
  bool is_zip = IsFileExtensions(
      filename, {".apk", ".zip", ".jar", ".zvoice", ".apex", "capex"});
  bool is_gzip = IsFileExtensions(filename, {".gz", ".gzip", ".tgz"});
  if (is_zip) {
    if (!puffin::LocateDeflatesInZipArchive(data, deflates)) {
      LOG(ERROR) << "Failed to locate deflates in zip file " << filename;
      deflates->clear();
      return false;
    }
  } else if (is_gzip) {
    if (!puffin::LocateDeflatesInGzip(data, deflates)) {
      LOG(ERROR) << "Failed to locate deflates in gzip file " << filename;
      deflates->clear();
      return false;
    }
  }
  return true;
}

bool PreprocessPartitionFiles(const PartitionConfig& part,
                              vector<FilesystemInterface::File>* result_files,
                              bool extract_deflates) {
  // Get the file system files.
  vector<FilesystemInterface::File> tmp_files;
  part.fs_interface->GetFiles(&tmp_files);
  result_files->reserve(tmp_files.size());

  for (auto& file : tmp_files) {
    auto is_regular_file = IsRegularFile(file);

    if (is_regular_file && extract_deflates && !file.is_compressed) {
      // Search for deflates if the file is in zip or gzip format.
      // .zvoice files may eventually move out of rootfs. If that happens,
      // remove ".zvoice" (crbug.com/782918).
      bool is_zip = IsFileExtensions(
          file.name, {".apk", ".zip", ".jar", ".zvoice", ".apex", "capex"});
      bool is_gzip = IsFileExtensions(file.name, {".gz", ".gzip", ".tgz"});
      if (is_zip || is_gzip) {
        brillo::Blob data;
        TEST_AND_RETURN_FALSE(utils::ReadExtents(
            part.path,
            file.extents,
            &data,
            kBlockSize * utils::BlocksInExtents(file.extents),
            kBlockSize));
        // |data| read from disk always has size multiple of kBlockSize. So it
        // might contain trailing garbage data and confuse the gzip/zip
        // processors. Trim them.
        if (file.file_stat.st_size > 0 &&
            static_cast<size_t>(file.file_stat.st_size) < data.size()) {
          data.resize(file.file_stat.st_size);
        }
        vector<puffin::BitExtent> deflates;
        if (!DeflatePreprocessFileData(file.name, data, &deflates)) {
          LOG(ERROR) << "Failed to preprocess deflate data in partition "
                     << part.name;
          return false;
        }
        // Shift the deflate's extent to the offset starting from the beginning
        // of the current partition; and the delta processor will align the
        // extents in a continuous buffer later.
        TEST_AND_RETURN_FALSE(
            ShiftBitExtentsOverExtents(file.extents, &deflates));
        file.deflates = std::move(deflates);
      }
    }

    result_files->push_back(file);
  }
  return true;
}

}  // namespace deflate_utils
}  // namespace chromeos_update_engine
