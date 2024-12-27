// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OutputRange, RandomAccessRange, SemiOutputRange};

/// Unstable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are NOT preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
/// - Unconventional name `sort_range_by` is used to avoid name clash with `sort_by`
///   of rust stdlib.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 5, 2, 1, 4];
/// let start = arr.start();
/// let end = arr.end();
/// algo::sort_range_by(&mut arr, start, end, |x, y| x < y);
/// assert!(arr.equals(&[1, 1, 2, 4, 5]));
/// ```
pub fn sort_range_by<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let n = rng.distance(start.clone(), end.clone());
    if n <= 16 {
        sort_details::insertion_sort(rng, start, end, is_less);
    } else {
        let quick_sort_depth = 2 * n.ilog2() as usize;
        if !sort_details::quick_sort_till_depth(
            rng,
            start.clone(),
            end.clone(),
            is_less.clone(),
            quick_sort_depth,
        ) {
            sort_details::heap_sort(rng, start, end, is_less);
        }
    }
}

/// Unstable sort: sorts range in non-decreasing order.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing order.
///   - Relative order of equivalent elements are NOT preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
/// - Unconventional name `sort_range` is used to avoid name clash with `sort`
///   of rust stdlib.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 5, 2, 1, 4];
/// let start = arr.start();
/// let end = arr.end();
/// algo::sort_range(&mut arr, start, end);
/// assert!(arr.equals(&[1, 1, 2, 4, 5]));
/// ```
pub fn sort_range<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    sort_range_by(rng, start, end, |x, y| x < y)
}

/// Stable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
/// - The algorithm allocates O(n) buffer for achieving O(n.log2(n)) time complexity.
///   If memory allocation is a problem, consider using `stable_sort_by_no_alloc`.
///   `stable_sort_by_no_alloc` doesn't do any buffer allocation, however provides
///   O(n.log2^2(n)) time complexity.
/// - The algorithm requires `OutputRange` trait to read and write from/to
///   buffer. If only `SemiOutputRange` is available, consider using
///   `stable_sort_by_no_alloc` with given tradeoffs.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// let start = arr.start();
/// let end = arr.end();
/// algo::stable_sort_by(&mut arr, start, end, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
/// ```
pub fn stable_sort_by<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + OutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let n = rng.distance(start.clone(), end.clone());
    if n <= 16 {
        sort_details::insertion_sort(rng, start, end, is_less);
    } else {
        let quick_sort_depth = 2 * n.ilog2() as usize;
        if !sort_details::stable_quick_sort_till_depth(
            rng,
            start.clone(),
            end.clone(),
            is_less.clone(),
            quick_sort_depth,
        ) {
            sort_details::merge_sort(rng, start, end, is_less);
        }
    }
}

/// Stable sort: sorts range in non-decreasing order.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
/// - The algorithm allocates O(n) buffer for achieving O(n.log2(n)) time complexity.
///   If memory allocation is a problem, consider using `stable_sort_no_alloc`.
///   `stable_sort_no_alloc` doesn't do any buffer allocation, however provides
///   O(n.log2^2(n)) time complexity.
/// - The algorithm requires `OutputRange` trait to read and write from/to
///   buffer. If only `SemiOutputRange` is available, consider using
///   `stable_sort_no_alloc` with given tradeoffs.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// let start = arr.start();
/// let end = arr.end();
/// algo::stable_sort(&mut arr, start, end);
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
/// ```
pub fn stable_sort<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) where
    Range: RandomAccessRange + OutputRange + ?Sized,
    Range::Element: Ord,
{
    stable_sort_by(rng, start, end, |x, y| x < y);
}

/// Stable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
/// - The algorithm provides O(n.log2^2(n)) time complexity. If memory allocation
///   is not a problem, consider using `stable_sort_by` algorithm.
/// - The algorithm just requires `SemiOutputRange` for mutation instead of
///   `OutputRange`, that might be desirable in some situations.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// let start = arr.start();
/// let end = arr.end();
/// algo::stable_sort_by_no_alloc(&mut arr, start, end, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
/// ```
pub fn stable_sort_by_no_alloc<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let n = rng.distance(start.clone(), end.clone());
    if n <= 16 {
        sort_details::insertion_sort(rng, start, end, is_less);
    } else {
        let quick_sort_depth = 2 * n.ilog2() as usize;
        if !sort_details::stable_quick_sort_till_depth_no_alloc(
            rng,
            start.clone(),
            end.clone(),
            is_less.clone(),
            quick_sort_depth,
        ) {
            sort_details::merge_sort_no_alloc(rng, start, end, is_less);
        }
    }
}

/// Stable sort: sorts range in non-decreasing order.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
/// - The algorithm provides O(n.log2^2(n)) time complexity. If memory allocation
///   is not a problem, consider using `stable_sort` algorithm.
/// - The algorithm just requires `SemiOutputRange` for mutation instead of
///   `OutputRange`, that might be desirable in some situations.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// let start = arr.start();
/// let end = arr.end();
/// algo::stable_sort_no_alloc(&mut arr, start, end);
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
/// ```
pub fn stable_sort_no_alloc<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    stable_sort_by_no_alloc(rng, start, end, |x, y| x < y);
}

// TODO: details can only be accessed from current file or from tests.
pub mod sort_details {
    use crate::{
        algo::{
            self, merge_inplace_by, merge_inplace_by_no_alloc,
            partition::partition_details::*, rotate,
        },
        OutputRange, RandomAccessRange, SemiOutputRange,
    };

    pub fn insertion_sort<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
    ) where
        Range: RandomAccessRange + SemiOutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool,
    {
        let mut i = start.clone();
        while i != end {
            let mut j = i.clone();
            while j != start
                && is_less(rng.at(&j), rng.at(&rng.before(j.clone())))
            {
                let prev = rng.before(j.clone());
                rng.swap_at(&prev, &j);
                j = prev;
            }
            i = rng.after(i);
        }
    }

    pub fn quick_sort_till_depth<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
        depth: usize,
    ) -> bool
    where
        Range: RandomAccessRange + SemiOutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
    {
        if start == end || rng.after(start.clone()) == end {
            return true;
        }

        if depth == 0 {
            return false;
        }

        let pivot = start.clone();
        let partition_start = rng.after(pivot.clone());
        let partition_point =
            partition_pos(rng, partition_start, end.clone(), |r, i| {
                is_less(r.at(i), r.at(&pivot))
            });
        let left_end = rng.before(partition_point.clone());
        rng.swap_at(&pivot, &left_end);
        let left = quick_sort_till_depth(
            rng,
            start,
            left_end,
            is_less.clone(),
            depth - 1,
        );
        let right = quick_sort_till_depth(
            rng,
            partition_point,
            end,
            is_less,
            depth - 1,
        );
        left && right
    }

    pub fn heap_sort<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
    ) where
        Range: RandomAccessRange + SemiOutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
    {
        algo::make_heap_by(rng, start.clone(), end.clone(), is_less.clone());
        algo::sort_heap_by(rng, start.clone(), end.clone(), is_less);
    }

    pub fn merge_sort<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
    ) where
        Range: RandomAccessRange + OutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
    {
        let n = rng.distance(start.clone(), end.clone());
        if n == 0 || n == 1 {
            return;
        }
        let mid = rng.after_n(start.clone(), n >> 1);
        merge_sort(rng, start.clone(), mid.clone(), is_less.clone());
        merge_sort(rng, mid.clone(), end.clone(), is_less.clone());
        merge_inplace_by(rng, start, mid, end, is_less);
    }

    // TODO: Do we need 2 pass:
    //   1. stable_partition
    //   2. rotate
    pub fn stable_quick_sort_till_depth<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
        depth: usize,
    ) -> bool
    where
        Range: RandomAccessRange + OutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
    {
        if start == end || rng.after(start.clone()) == end {
            return true;
        }

        if depth == 0 {
            return false;
        }

        let pivot = start.clone();
        let partition_start = rng.after(pivot.clone());
        let partition_point = stable_partition_pos(
            rng,
            partition_start.clone(),
            end.clone(),
            |r, i| is_less(r.at(i), r.at(&pivot)),
        );
        let left_end = rng.before(partition_point.clone());
        rotate(rng, pivot, partition_start, partition_point.clone());
        let left = stable_quick_sort_till_depth(
            rng,
            start,
            left_end,
            is_less.clone(),
            depth - 1,
        );
        let right = stable_quick_sort_till_depth(
            rng,
            partition_point,
            end,
            is_less,
            depth - 1,
        );
        left && right
    }

    pub fn merge_sort_no_alloc<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
    ) where
        Range: RandomAccessRange + SemiOutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
    {
        let n = rng.distance(start.clone(), end.clone());
        if n == 0 || n == 1 {
            return;
        }
        let mid = rng.after_n(start.clone(), n >> 1);
        merge_sort_no_alloc(rng, start.clone(), mid.clone(), is_less.clone());
        merge_sort_no_alloc(rng, mid.clone(), end.clone(), is_less.clone());
        merge_inplace_by_no_alloc(rng, start, mid, end, is_less);
    }

    // TODO: Do we need 2 pass:
    //   1. stable_partition
    //   2. rotate
    pub fn stable_quick_sort_till_depth_no_alloc<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
        depth: usize,
    ) -> bool
    where
        Range: RandomAccessRange + SemiOutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
    {
        if start == end || rng.after(start.clone()) == end {
            return true;
        }

        if depth == 0 {
            return false;
        }

        let pivot = start.clone();
        let partition_start = rng.after(pivot.clone());
        let partition_point = stable_partition_no_alloc_pos(
            rng,
            partition_start.clone(),
            end.clone(),
            |r, i| is_less(r.at(i), r.at(&pivot)),
        );
        let left_end = rng.before(partition_point.clone());
        rotate(rng, pivot, partition_start, partition_point.clone());
        let left = stable_quick_sort_till_depth_no_alloc(
            rng,
            start,
            left_end,
            is_less.clone(),
            depth - 1,
        );
        let right = stable_quick_sort_till_depth_no_alloc(
            rng,
            partition_point,
            end,
            is_less,
            depth - 1,
        );
        left && right
    }
}
