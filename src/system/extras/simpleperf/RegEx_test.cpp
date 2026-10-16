/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "RegEx.h"

#include <gtest/gtest.h>

using namespace simpleperf;

// @CddTest = 6.1/C-0-2
TEST(RegEx, smoke) {
  auto re = RegEx::Create("b+");
  ASSERT_EQ(re->GetPattern(), "b+");
  ASSERT_FALSE(re->Search("aaa"));
  ASSERT_TRUE(re->Search("aba"));
  ASSERT_FALSE(re->Match("aba"));
  ASSERT_TRUE(re->Match("bbb"));

  ASSERT_FALSE(re->ThreadUnsafeSearch("aaa"));
  ASSERT_TRUE(re->ThreadUnsafeSearch("aba"));
  ASSERT_FALSE(re->ThreadUnsafeMatch("aba"));
  ASSERT_TRUE(re->ThreadUnsafeMatch("bbb"));

  auto match = re->SearchAll("aaa");
  ASSERT_FALSE(match->IsValid());
  match = re->SearchAll("ababb");
  ASSERT_TRUE(match->IsValid());
  ASSERT_EQ(match->GetField(0), "b");
  match->MoveToNextMatch();
  ASSERT_TRUE(match->IsValid());
  ASSERT_EQ(match->GetField(0), "bb");
  match->MoveToNextMatch();
  ASSERT_FALSE(match->IsValid());

  ASSERT_EQ(re->Replace("ababb", "c").value(), "acac");
}

// @CddTest = 6.1/C-0-2
TEST(RegEx, invalid_pattern) {
  ASSERT_TRUE(RegEx::Create("?hello") == nullptr);
}

// @CddTest = 6.1/C-0-2
TEST(RegEx, SearchAll_with_capture_groups) {
  // Tests SearchAll with multiple matches and capture groups.
  auto re = RegEx::Create("(a+)(b+)");
  ASSERT_TRUE(re != nullptr);

  auto match = re->SearchAll("xx aabb yy aaab zz");

  // First match: "aabb"
  ASSERT_TRUE(match->IsValid());
  EXPECT_EQ(match->GetField(0), "aabb");
  EXPECT_EQ(match->GetField(1), "aa");
  EXPECT_EQ(match->GetField(2), "bb");
  // Test out-of-bounds index.
  EXPECT_EQ(match->GetField(3), "");

  match->MoveToNextMatch();

  // Second match: "aaab"
  ASSERT_TRUE(match->IsValid());
  EXPECT_EQ(match->GetField(0), "aaab");
  EXPECT_EQ(match->GetField(1), "aaa");
  EXPECT_EQ(match->GetField(2), "b");

  match->MoveToNextMatch();

  // No more matches.
  ASSERT_FALSE(match->IsValid());
  // GetField on an invalid match should return empty string.
  EXPECT_EQ(match->GetField(0), "");
}

// @CddTest = 6.1/C-0-2
TEST(RegEx, SearchAll_with_optional_capture_group) {
  // Tests an optional capture group that may or may not match.
  auto re = RegEx::Create("(a)(b)?(c)");
  ASSERT_TRUE(re != nullptr);

  auto match = re->SearchAll("x ac y abc z");

  // First match: "ac", where the optional group (b) is not present.
  ASSERT_TRUE(match->IsValid());
  EXPECT_EQ(match->GetField(0), "ac");
  EXPECT_EQ(match->GetField(1), "a");
  EXPECT_EQ(match->GetField(2), "");  // Optional group did not match.
  EXPECT_EQ(match->GetField(3), "c");

  match->MoveToNextMatch();

  // Second match: "abc", where the optional group (b) is present.
  ASSERT_TRUE(match->IsValid());
  EXPECT_EQ(match->GetField(0), "abc");
  EXPECT_EQ(match->GetField(1), "a");
  EXPECT_EQ(match->GetField(2), "b");
  EXPECT_EQ(match->GetField(3), "c");

  match->MoveToNextMatch();
  ASSERT_FALSE(match->IsValid());
}

// @CddTest = 6.1/C-0-2
TEST(RegEx, Replace_features) {
  auto re = RegEx::Create("(a+)(b+)");
  // Test replacement with backreferences ($2$1 swaps the capture groups).
  ASSERT_EQ(re->Replace("xx aabb yy aaab", "$2$1").value(), "xx bbaa yy baaa");
  // Test replacement on a string with no matches (should return original string).
  ASSERT_EQ(re->Replace("xx c dd", "$2$1").value(), "xx c dd");
}
