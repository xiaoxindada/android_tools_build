// Copyright (C) 2018 The Android Open Source Project
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

#include <optional>
#include <tuple>

#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <libdm/dm.h>
#include <liblp/builder.h>
#include <liblp/property_fetcher.h>

#include <libsnapshot/test_helpers.h>

#include "partition_cow_creator.h"
#include "utility.h"

using namespace android::fs_mgr;

using chromeos_update_engine::InstallOperation;
using UeExtent = chromeos_update_engine::Extent;
using google::protobuf::RepeatedPtrField;
using ::testing::Matches;
using ::testing::Pointwise;
using ::testing::Truly;

namespace android {
namespace snapshot {

// @VsrTest = 3.7.6
class PartitionCowCreatorTest : public ::testing::Test {
  public:
    void SetUp() override {
        SKIP_IF_NON_VIRTUAL_AB();
        SnapshotTestPropertyFetcher::SetUp();
    }
    void TearDown() override {
        RETURN_IF_NON_VIRTUAL_AB();
        SnapshotTestPropertyFetcher::TearDown();
    }
};

TEST_F(PartitionCowCreatorTest, IntersectSelf) {
    constexpr uint64_t super_size = 1_MiB;
    constexpr uint64_t partition_size = 40_KiB;

    auto builder_a = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_a, nullptr);
    auto system_a = builder_a->AddPartition("system_a", LP_PARTITION_ATTR_READONLY);
    ASSERT_NE(system_a, nullptr);
    ASSERT_TRUE(builder_a->ResizePartition(system_a, partition_size));

    auto builder_b = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_b, nullptr);
    auto system_b = builder_b->AddPartition("system_b", LP_PARTITION_ATTR_READONLY);
    ASSERT_NE(system_b, nullptr);
    ASSERT_TRUE(builder_b->ResizePartition(system_b, partition_size));

    PartitionUpdate update;
    // this value here can be anything, PartitionCowCreator just requires an estimate
    // cow size from the manifest
    update.set_estimate_cow_size(1_MiB);

    PartitionCowCreator creator{.target_metadata = builder_b.get(),
                                .target_suffix = "_b",
                                .target_partition = system_b,
                                .current_metadata = builder_a.get(),
                                .current_suffix = "_a",
                                .update = &update};
    auto ret = creator.Run();
    ASSERT_TRUE(ret.has_value());
    ASSERT_EQ(partition_size, ret->snapshot_status.device_size());
    ASSERT_EQ(partition_size, ret->snapshot_status.snapshot_size());
}

TEST_F(PartitionCowCreatorTest, Holes) {
    const auto& opener = test_device->GetPartitionOpener();

    constexpr auto slack_space = 1_MiB;
    constexpr auto big_size = (kSuperSize - slack_space) / 2;
    constexpr auto small_size = big_size / 2;

    BlockDeviceInfo super_device("super", kSuperSize, 0, 0, 4_KiB);
    std::vector<BlockDeviceInfo> devices = {super_device};
    auto source = MetadataBuilder::New(devices, "super", 1_KiB, 2);
    auto system = source->AddPartition("system_a", 0);
    ASSERT_NE(nullptr, system);
    ASSERT_TRUE(source->ResizePartition(system, big_size));
    auto vendor = source->AddPartition("vendor_a", 0);
    ASSERT_NE(nullptr, vendor);
    ASSERT_TRUE(source->ResizePartition(vendor, big_size));
    // Create a hole between system and vendor
    ASSERT_TRUE(source->ResizePartition(system, small_size));
    auto source_metadata = source->Export();
    ASSERT_NE(nullptr, source_metadata);
    ASSERT_TRUE(FlashPartitionTable(opener, fake_super, *source_metadata.get()));

    auto target = MetadataBuilder::NewForUpdate(opener, "super", 0, 1);
    // Shrink vendor
    vendor = target->FindPartition("vendor_b");
    ASSERT_NE(nullptr, vendor);
    ASSERT_TRUE(target->ResizePartition(vendor, small_size));
    // Grow system to take hole & saved space from vendor
    system = target->FindPartition("system_b");
    ASSERT_NE(nullptr, system);
    ASSERT_TRUE(target->ResizePartition(system, big_size * 2 - small_size));

    PartitionUpdate update;
    // this value here can be anything, PartitionCowCreator just requires an estimate
    // cow size from the manifest
    update.set_estimate_cow_size(1_MiB);

    PartitionCowCreator creator{.target_metadata = target.get(),
                                .target_suffix = "_b",
                                .target_partition = system,
                                .current_metadata = source.get(),
                                .current_suffix = "_a",
                                .update = &update};
    auto ret = creator.Run();
    ASSERT_TRUE(ret.has_value());
}

TEST_F(PartitionCowCreatorTest, Zero) {
    constexpr uint64_t super_size = 1_MiB;
    auto builder_a = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_a, nullptr);

    auto builder_b = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_b, nullptr);
    auto system_b = builder_b->AddPartition("system_b", LP_PARTITION_ATTR_READONLY);
    ASSERT_NE(system_b, nullptr);

    PartitionCowCreator creator{.target_metadata = builder_b.get(),
                                .target_suffix = "_b",
                                .target_partition = system_b,
                                .current_metadata = builder_a.get(),
                                .current_suffix = "_a",
                                .update = nullptr};

    auto ret = creator.Run();

    ASSERT_EQ(0u, ret->snapshot_status.device_size());
    ASSERT_EQ(0u, ret->snapshot_status.snapshot_size());
    ASSERT_EQ(0u, ret->snapshot_status.cow_file_size());
    ASSERT_EQ(0u, ret->snapshot_status.cow_partition_size());
}

TEST_F(PartitionCowCreatorTest, CompressionEnabled) {
    constexpr uint64_t super_size = 1_MiB;
    auto builder_a = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_a, nullptr);

    auto builder_b = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_b, nullptr);
    auto system_b = builder_b->AddPartition("system_b", LP_PARTITION_ATTR_READONLY);
    ASSERT_NE(system_b, nullptr);
    ASSERT_TRUE(builder_b->ResizePartition(system_b, 128_KiB));

    PartitionUpdate update;
    update.set_estimate_cow_size(256_KiB);

    PartitionCowCreator creator{.target_metadata = builder_b.get(),
                                .target_suffix = "_b",
                                .target_partition = system_b,
                                .current_metadata = builder_a.get(),
                                .current_suffix = "_a",
                                .update = &update,
                                .using_snapuserd = true};

    auto ret = creator.Run();
    ASSERT_TRUE(ret.has_value());
    ASSERT_EQ(ret->snapshot_status.cow_file_size(), 1458176);
}

TEST_F(PartitionCowCreatorTest, CompressionWithNoManifest) {
    constexpr uint64_t super_size = 1_MiB;
    auto builder_a = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_a, nullptr);

    auto builder_b = MetadataBuilder::New(super_size, 1_KiB, 2);
    ASSERT_NE(builder_b, nullptr);
    auto system_b = builder_b->AddPartition("system_b", LP_PARTITION_ATTR_READONLY);
    ASSERT_NE(system_b, nullptr);
    ASSERT_TRUE(builder_b->ResizePartition(system_b, 128_KiB));

    PartitionUpdate update;

    PartitionCowCreator creator{.target_metadata = builder_b.get(),
                                .target_suffix = "_b",
                                .target_partition = system_b,
                                .current_metadata = builder_a.get(),
                                .current_suffix = "_a",
                                .update = nullptr,
                                .using_snapuserd = true};

    auto ret = creator.Run();
    ASSERT_FALSE(ret.has_value());
}

void BlocksToExtents(const std::vector<uint64_t>& blocks,
                     google::protobuf::RepeatedPtrField<UeExtent>* extents) {
    for (uint64_t block : blocks) {
        AppendExtent(extents, block, 1);
    }
}

template <typename T>
std::vector<uint64_t> ExtentsToBlocks(const T& extents) {
    std::vector<uint64_t> blocks;
    for (const auto& extent : extents) {
        for (uint64_t offset = 0; offset < extent.num_blocks(); ++offset) {
            blocks.push_back(extent.start_block() + offset);
        }
    }
    return blocks;
}

InstallOperation CreateCopyOp(const std::vector<uint64_t>& src_blocks,
                              const std::vector<uint64_t>& dst_blocks) {
    InstallOperation op;
    op.set_type(InstallOperation::SOURCE_COPY);
    BlocksToExtents(src_blocks, op.mutable_src_extents());
    BlocksToExtents(dst_blocks, op.mutable_dst_extents());
    return op;
}

// ExtentEqual(tuple<UeExtent, UeExtent>)
MATCHER(ExtentEqual, "") {
    auto&& [a, b] = arg;
    return a.start_block() == b.start_block() && a.num_blocks() == b.num_blocks();
}

struct OptimizeOperationTestParam {
    InstallOperation input;
    std::optional<InstallOperation> expected_output;
};

class OptimizeOperationTest : public ::testing::TestWithParam<OptimizeOperationTestParam> {
    void SetUp() override { SKIP_IF_NON_VIRTUAL_AB(); }
};
TEST_P(OptimizeOperationTest, Test) {
    InstallOperation actual_output;
    EXPECT_EQ(GetParam().expected_output.has_value(),
              OptimizeSourceCopyOperation(GetParam().input, &actual_output))
            << "OptimizeSourceCopyOperation should "
            << (GetParam().expected_output.has_value() ? "succeed" : "fail");
    if (!GetParam().expected_output.has_value()) return;
    EXPECT_THAT(actual_output.src_extents(),
                Pointwise(ExtentEqual(), GetParam().expected_output->src_extents()));
    EXPECT_THAT(actual_output.dst_extents(),
                Pointwise(ExtentEqual(), GetParam().expected_output->dst_extents()));
}

std::vector<OptimizeOperationTestParam> GetOptimizeOperationTestParams() {
    return {
            {CreateCopyOp({}, {}), CreateCopyOp({}, {})},
            {CreateCopyOp({1, 2, 4}, {1, 2, 4}), CreateCopyOp({}, {})},
            {CreateCopyOp({1, 2, 3}, {4, 5, 6}), std::nullopt},
            {CreateCopyOp({3, 2}, {1, 2}), CreateCopyOp({3}, {1})},
            {CreateCopyOp({5, 6, 3, 4, 1, 2}, {1, 2, 3, 4, 5, 6}),
             CreateCopyOp({5, 6, 1, 2}, {1, 2, 5, 6})},
            {CreateCopyOp({1, 2, 3, 5, 5, 6}, {5, 6, 3, 4, 1, 2}),
             CreateCopyOp({1, 2, 5, 5, 6}, {5, 6, 4, 1, 2})},
            {CreateCopyOp({1, 2, 5, 6, 9, 10}, {1, 4, 5, 6, 7, 8}),
             CreateCopyOp({2, 9, 10}, {4, 7, 8})},
            {CreateCopyOp({2, 3, 3, 4, 4}, {1, 2, 3, 4, 5}), CreateCopyOp({2, 3, 4}, {1, 2, 5})},
    };
}

INSTANTIATE_TEST_CASE_P(Snapshot, OptimizeOperationTest,
                        ::testing::ValuesIn(GetOptimizeOperationTestParams()));

}  // namespace snapshot
}  // namespace android
