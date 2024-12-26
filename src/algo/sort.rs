// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{RandomAccessRange, SemiOutputRange};

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
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    details::insertion_sort(rng, start, end, is_less);
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

// TODO: details can only be accessed from current file or from tests.
pub mod details {
    use crate::{algo::partition, RandomAccessRange, SemiOutputRange};

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
        let pivot_ele = unsafe { std::ptr::read(rng.at(&pivot)) };
        let partition_start = rng.after(pivot.clone());
        let partition_point =
            partition(rng, partition_start, end.clone(), |x| {
                is_less(x, &pivot_ele)
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
}
