//
// Copyright (C) 2026 The Android Open Source Project
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

#include "libsnapshot_cow/extent_map.h"
#include <gtest/gtest.h>

namespace android {
namespace snapshot {

class ExtentMapTest : public ::testing::Test {
  protected:
    void SetUp() override {}
    void TearDown() override {}
};

TEST_F(ExtentMapTest, BasicInsertAndGet) {
    ExtentMap<Extent, int> map;
    Extent e1(10, 5);
    ASSERT_TRUE(map.Insert(e1, 100));
    ASSERT_EQ(map.Size(), 1);

    auto val = map.Get(e1);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 100);

    val = map.Get(12);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 100);

    val = map.Get(9);
    ASSERT_FALSE(val.has_value());

    val = map.Get(15);  // 10 + 5 = 15, so 15 is excluded (range [10, 15))
    ASSERT_FALSE(val.has_value());

    // Test Get(Extent) partial overlap or sub-range
    Extent e_sub(12, 1);
    val = map.Get(e_sub);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 100);
}

TEST_F(ExtentMapTest, OverlappingInsert) {
    ExtentMap<Extent, int> map;
    Extent e1(10, 5);
    ASSERT_TRUE(map.Insert(e1, 100));

    Extent e2(12, 1);
    ASSERT_FALSE(map.Insert(e2, 200));
    ASSERT_EQ(map.Size(), 1);

    Extent e3(8, 3);  // Overlaps at 10
    ASSERT_FALSE(map.Insert(e3, 300));
}

struct SumMerge {
    std::optional<int> Merge(const int& a, const int& b) const { return a + b; }
};

TEST_F(ExtentMapTest, MergeExtents) {
    ExtentMap<Extent, int, SumMerge> map;
    Extent e1(10, 5);
    ASSERT_TRUE(map.Insert(e1, 100));

    Extent e2(15, 5);  // Adjacent, touches at 15
    ASSERT_TRUE(map.Insert(e2, 200));

    // Should merge into [10, 20) with value 300
    ASSERT_EQ(map.Size(), 1);
    auto val = map.Get(12);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 300);

    val = map.Get(17);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 300);

    // Check bounds
    Extent full(10, 10);
    val = map.Get(full);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 300);
}

TEST_F(ExtentMapTest, NoMergeIfNotTouching) {
    ExtentMap<Extent, int, SumMerge> map;
    Extent e1(10, 5);
    ASSERT_TRUE(map.Insert(e1, 100));

    Extent e2(16, 5);  // Gap at 15
    ASSERT_TRUE(map.Insert(e2, 200));

    ASSERT_EQ(map.Size(), 2);
    auto val = map.Get(12);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 100);

    val = map.Get(17);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 200);
}

TEST_F(ExtentMapTest, InsertSmallest) {
    ExtentMap<Extent, int> map;
    ASSERT_TRUE(map.Insert(Extent(50, 10), 100));
    ASSERT_TRUE(map.Insert(Extent(10, 10), 200));  // Smallest
    ASSERT_EQ(map.Size(), 2);
    auto val = map.Get(10);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 200);
}

TEST_F(ExtentMapTest, InsertLargest) {
    ExtentMap<Extent, int> map;
    ASSERT_TRUE(map.Insert(Extent(10, 10), 100));
    ASSERT_TRUE(map.Insert(Extent(50, 10), 200));  // Largest
    ASSERT_EQ(map.Size(), 2);
    auto val = map.Get(50);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 200);
}

TEST_F(ExtentMapTest, InsertMiddle) {
    ExtentMap<Extent, int> map;
    ASSERT_TRUE(map.Insert(Extent(10, 10), 100));
    ASSERT_TRUE(map.Insert(Extent(50, 10), 200));
    ASSERT_TRUE(map.Insert(Extent(30, 10), 300));  // Middle
    ASSERT_EQ(map.Size(), 3);
    auto val = map.Get(35);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 300);
}

TEST_F(ExtentMapTest, OverlapVarious) {
    ExtentMap<Extent, int> map;
    ASSERT_TRUE(map.Insert(Extent(50, 10), 100));

    // Overlap at exact same start block
    ASSERT_FALSE(map.Insert(Extent(50, 5), 200));

    // Overlap where new extent starts before and ends inside
    ASSERT_FALSE(map.Insert(Extent(45, 10), 300));

    // Overlap where new extent starts inside and ends after
    ASSERT_FALSE(map.Insert(Extent(55, 10), 400));

    // Overlap where new extent completely contains the existing one
    ASSERT_FALSE(map.Insert(Extent(40, 30), 500));

    // Overlap where new extent is completely inside the existing one
    ASSERT_FALSE(map.Insert(Extent(52, 5), 600));

    // Touch but not overlap (before)
    ASSERT_TRUE(map.Insert(Extent(40, 10), 700));

    // Touch but not overlap (after)
    ASSERT_TRUE(map.Insert(Extent(60, 10), 800));
}

TEST_F(ExtentMapTest, MergeSmallestLargestMiddle) {
    ExtentMap<Extent, int, SumMerge> map;

    // Setup initial non-contiguous extents
    ASSERT_TRUE(map.Insert(Extent(10, 5), 100));
    ASSERT_TRUE(map.Insert(Extent(30, 5), 200));
    ASSERT_TRUE(map.Insert(Extent(50, 5), 300));
    ASSERT_EQ(map.Size(), 3);

    // Merge at the smallest block number (prepend)
    ASSERT_TRUE(map.Insert(Extent(5, 5), 50));
    ASSERT_EQ(map.Size(), 3);
    auto val = map.Get(7);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 150);

    // Merge at the largest block number (append)
    ASSERT_TRUE(map.Insert(Extent(55, 5), 400));
    ASSERT_EQ(map.Size(), 3);
    val = map.Get(57);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 700);

    // Merge in the middle (append context)
    ASSERT_TRUE(map.Insert(Extent(35, 5), 500));
    ASSERT_EQ(map.Size(), 3);
    val = map.Get(37);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 700);
}

TEST_F(ExtentMapTest, UpperBoundEdgeCases) {
    ExtentMap<Extent, int> map;
    ASSERT_TRUE(map.Insert(Extent(10, 10), 100));
    ASSERT_TRUE(map.Insert(Extent(20, 5), 200));

    auto val = map.Get(Extent(5, 5));
    ASSERT_FALSE(val.has_value());

    val = map.Get(Extent(25, 5));
    ASSERT_FALSE(val.has_value());
}

TEST_F(ExtentMapTest, MergeThreeExtents) {
    ExtentMap<Extent, int, SumMerge> map;
    ASSERT_TRUE(map.Insert(Extent(10, 5), 100));
    ASSERT_TRUE(map.Insert(Extent(20, 5), 100));
    ASSERT_TRUE(map.Insert(Extent(15, 5), 100));

    ASSERT_EQ(map.Size(), 1);
    auto val = map.Get(10);
    ASSERT_TRUE(val.has_value());
    ASSERT_EQ(val.value(), 300);
    ASSERT_TRUE(map.Get(24).has_value());
}

}  // namespace snapshot
}  // namespace android
