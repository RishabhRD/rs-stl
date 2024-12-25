// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::ForwardRange;

use super::partition_point;

/// Returns the position of first element in partitioned range which is not ordered before value
/// wrt comparator.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `is_less(rng.at(&i), ele)`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng st `is_less(rng.at(&i), ele) == false`.
///   - If no such element exists, return end.
///   - Complexity: O(log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 1, 4, 8, 7];
/// let i = algo::lower_bound_by(&arr, arr.start(), arr.end(), &4, |x, y| x < y);
/// assert_eq!(i, 2);
/// ```
pub fn lower_bound_by<Range, Compare>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    ele: &Range::Element,
    is_less: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    partition_point(rng, start, end, |x| is_less(x, ele))
}

/// Returns the position of first element in partitioned range which is not ordered before value.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `rng.at(&i) < ele`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng st `rng.at(&i) >= ele`.
///   - If no such element exists, return end.
///   - Complexity: O(log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 1, 4, 8, 7];
/// let i = algo::lower_bound(&arr, arr.start(), arr.end(), &4);
/// assert_eq!(i, 2);
/// ```
pub fn lower_bound<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    ele: &Range::Element,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    lower_bound_by(rng, start, end, ele, |x, y| x < y)
}
