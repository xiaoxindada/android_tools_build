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

#pragma once
#include <stdint.h>
#include <concepts>
#include <iterator>
#include <map>
#include <optional>

template <typename T>
concept ExtentLike = requires(T a, size_t b) {
    { static_cast<const T&>(a).start_block() } -> std::convertible_to<size_t>;
    { static_cast<const T&>(a).num_blocks() } -> std::convertible_to<size_t>;
    { T() };
    { a.set_start_block(b) };
    { a.set_num_blocks(b) };
};

template <typename T>
struct Range {
    constexpr Range(T begin, T end) : begin_(begin), end_(end) {}
    constexpr auto begin() const noexcept { return begin_; }
    constexpr auto end() const noexcept { return end_; }
    T begin_;
    T end_;
};

template <ExtentLike T>
struct ExtentLess {
    constexpr bool operator()(const T& x, const T& y) const {
        if (x.start_block() == y.start_block()) {
            return x.num_blocks() < y.num_blocks();
        }
        return x.start_block() < y.start_block();
    }
};

template <ExtentLike Extent>
constexpr bool ExtentContains(const Extent& extent, uint64_t block) {
    return extent.start_block() <= block && block < extent.start_block() + extent.num_blocks();
}

template <ExtentLike Extent>
constexpr bool ExtentTouching(const Extent& a, const Extent& b) {
    return a.start_block() + a.num_blocks() == b.start_block() ||
           b.start_block() + b.num_blocks() == a.start_block();
}

// return true iff |big| extent contains |small| extent
template <ExtentLike Extent>
constexpr bool ExtentContains(const Extent& big, const Extent& small) {
    return big.start_block() <= small.start_block() &&
           small.start_block() + small.num_blocks() <= big.start_block() + big.num_blocks();
}

template <ExtentLike Extent>
bool ExtentsOverlap(const Extent& a, const Extent& b) {
    // Empty extents cannot overlap
    if (a.num_blocks() == 0 || b.num_blocks() == 0) {
        return false;
    }
    if (a.start_block() < b.start_block()) {
        // Check if A ends after B starts: a.start + a.num > b.start
        // Rewritten to avoid overflow: a.num > b.start - a.start
        return a.num_blocks() > b.start_block() - a.start_block();
    } else {
        // Check if B ends after A starts: b.start + b.num > a.start
        // Rewritten to avoid overflow: b.num > a.start - b.start
        return b.num_blocks() > a.start_block() - b.start_block();
    }
}

template <ExtentLike Extent>
bool ExtentsOverlapOrTouch(const Extent& a, const Extent& b) {
    // Empty extents cannot overlap/touch
    if (a.num_blocks() == 0 || b.num_blocks() == 0) {
        return false;
    }
    if (a.start_block() < b.start_block()) {
        // Check if A ends after B starts: a.start + a.num >= b.start
        // Rewritten to avoid overflow: a.num >= b.start - a.start
        return a.num_blocks() >= b.start_block() - a.start_block();
    } else {
        // Check if B ends after A starts: b.start + b.num >= a.start
        // Rewritten to avoid overflow: b.num >= a.start - b.start
        return b.num_blocks() >= a.start_block() - b.start_block();
    }
}

template <ExtentLike Extent>
Extent ExtentForRange(uint64_t start_block, uint64_t num_blocks) {
    Extent ret;
    ret.set_start_block(start_block);
    ret.set_num_blocks(num_blocks);
    return ret;
}

struct Extent {
    uint64_t start_block_;
    uint64_t num_blocks_;
    constexpr Extent(uint64_t block, uint64_t count) : start_block_(block), num_blocks_(count) {}
    constexpr Extent() : start_block_(0), num_blocks_(0) {}
    constexpr uint64_t start_block() const { return start_block_; }
    constexpr uint64_t num_blocks() const { return num_blocks_; }
    constexpr void set_start_block(uint64_t block) { start_block_ = block; }
    constexpr void set_num_blocks(uint64_t count) { num_blocks_ = count; }
};

template <typename T>
struct NoMerge {
    constexpr std::optional<T> Merge(const T&, const T&) const { return {}; }
};

template <typename T, typename U>
concept Merger = requires(const T& t, const U& u) {
    { t.Merge(u, u) } -> std::same_as<std::optional<U>>;
};

template <ExtentLike T, typename V, typename Merge = NoMerge<V>>
    requires Merger<Merge, V>
class ExtentMap {
  public:
    ExtentMap(const Merge& merge) : merge_(merge) {}
    ExtentMap() : merge_() {}
    using MapType = std::map<T, V, ExtentLess<T>>;

    Range<typename MapType::const_iterator> GetCandidateRange(const T& extent) const {
        auto lower_it = map_.lower_bound(extent);
        // we only store unique and non-overlapping extents,
        // so at most 1 extent before lower_it can overlap with `extent`
        if (lower_it != map_.begin() && ExtentsOverlapOrTouch(std::prev(lower_it)->first, extent)) {
            --lower_it;
        }
        // UINT64_MAX because we do want to include extents starting at
        // extent.start_block() + extent.num_blocks() in the range, the
        // dummy element specifies the "largest" extent we would like to
        // include in the returned range.
        auto upper_it = map_.upper_bound(
                ExtentForRange<T>(extent.start_block() + extent.num_blocks(), UINT64_MAX));
        return {lower_it, upper_it};
    }
    bool Insert(const T& extent_in, const V& value_in) {
        if (map_.find(extent_in) != map_.end()) {
            return false;
        }

        T new_ext = extent_in;
        V new_val = value_in;
        auto range = GetCandidateRange(extent_in);

        auto it = range.begin();
        while (it != range.end()) {
            const auto& key = it->first;
            const auto& val = it->second;

            if (ExtentsOverlap(key, extent_in)) {
                return false;
            }

            bool erased = false;
            if (key.start_block() + key.num_blocks() == new_ext.start_block()) {
                auto merged_val = merge_.Merge(val, new_val);
                if (merged_val.has_value()) {
                    new_ext.set_start_block(key.start_block());
                    new_ext.set_num_blocks(key.num_blocks() + new_ext.num_blocks());
                    new_val = std::move(merged_val.value());
                    it = map_.erase(it);
                    erased = true;
                }
            } else if (new_ext.start_block() + new_ext.num_blocks() == key.start_block()) {
                auto merged_val = merge_.Merge(new_val, val);
                if (merged_val.has_value()) {
                    new_ext.set_num_blocks(new_ext.num_blocks() + key.num_blocks());
                    new_val = std::move(merged_val.value());
                    it = map_.erase(it);
                    erased = true;
                }
            }

            if (!erased) {
                ++it;
            }
        }

        map_.insert({new_ext, new_val});
        return true;
    }

    std::optional<V> Get(const T& extent) const {
        for (const auto& [key, val] : GetCandidateRange(extent)) {
            if (ExtentContains(key, extent)) {
                return val;
            }
        }
        return {};
    }

    std::optional<V> Get(uint64_t block) const {
        T t;
        t.set_start_block(block);
        t.set_num_blocks(1);
        return Get(t);
    }

    size_t Size() const { return map_.size(); }
    bool Empty() const { return map_.empty(); }

  private:
    MapType map_;
    Merge merge_;
};