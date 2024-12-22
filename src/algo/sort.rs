// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{RandomAccessRange, SemiOutputRange};

use super::details::insertion_sort;

/// unstable sort: sort range in non-decreasing order based on comparator.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///   - cmp follows strict weak ordering relationship, i.e., returns true for
///     cmp(a, b) if a comes before b otherwise false.
///   - Also if cmp(a, b) == false && cmp(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing order by comparator cmp.
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
    cmp: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    insertion_sort(rng, start, end, cmp);
}

/// unstable sort: sort range in non-decreasing order based.
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
