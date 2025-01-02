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
///   - Returns position `i` of first element in rng at `[start, end)` st `is_less(rng.at(&i), ele) == false`.
///   - If no such element exists, returns end.
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
///   - Returns position `i` of first element in rng at `[start, end)` st `rng.at(&i) >= ele`.
///   - If no such element exists, returns end.
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

/// Returns the position of first element in partitioned range which is not ordered after value
/// wrt comparator.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `is_less(rng.at(&i), ele)`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng at `[start, end)` st `is_less(ele, rng.at(&i)) == true`.
///   - If no such element exists, returns end.
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
/// let i = algo::upper_bound_by(&arr, arr.start(), arr.end(), &4, |x, y| x < y);
/// assert_eq!(i, 3);
/// ```
pub fn upper_bound_by<Range, Compare>(
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
    partition_point(rng, start, end, |x| !is_less(ele, x))
}

/// Returns the position of first element in partitioned range which is not ordered after value.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `rng.at(&i) < ele`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng at `[start, end)` st `rng.at(&i) > ele`.
///   - If no such element exists, returns end.
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
///
/// let i = algo::upper_bound(&arr, arr.start(), arr.end(), &4);
/// assert_eq!(i, 3);
/// ```
pub fn upper_bound<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    ele: &Range::Element,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    upper_bound_by(rng, start, end, ele, |x, y| x < y)
}

/// Returns a pair of positions representing position of all elements equivalent to ele in partitioned range wrt comparator.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `is_less(rng.at(&i), ele)`.
///
/// # Postcondition
///   - Returns pair of position such that:
///     1. first position is position of first element in rng at `[start, end)` not ordered before
///        value wrt is_less. If no such element is found then end position.
///     2. second position is position of first element in rng at `[start, end)` ordered after
///        value wrt is_less. If no such element is found then end position.
///   - Complexity: O(2.log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 1, 4, 4, 8, 7];
/// let (i, j) = algo::equal_range_by(&arr, arr.start(), arr.end(), &4, |x, y| x < y);
/// assert_eq!(i, 2);
/// assert_eq!(j, 4);
/// ```
pub fn equal_range_by<Range, Compare>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    ele: &Range::Element,
    is_less: Compare,
) -> (Range::Position, Range::Position)
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let i = lower_bound_by(rng, start, end.clone(), ele, is_less.clone());
    let j = upper_bound_by(rng, i.clone(), end, ele, is_less);
    (i, j)
}

/// Returns a pair of positions representing position of all elements equal to ele in partitioned range.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `rng.at(&i) < ele`.
///
/// # Postcondition
///   - Returns pair of position such that:
///     1. first position is position of first element in rng at `[start, end)` not ordered before
///        value. If no such element is found then end position.
///     2. second position is position of first element in rng at `[start, end)` ordered after
///        value. If no such element is found then end position.
///   - Complexity: O(2.log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 1, 4, 4, 8, 7];
/// let (i, j) = algo::equal_range(&arr, arr.start(), arr.end(), &4);
/// assert_eq!(i, 2);
/// assert_eq!(j, 4);
/// ```
pub fn equal_range<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    ele: &Range::Element,
) -> (Range::Position, Range::Position)
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    equal_range_by(rng, start, end, ele, |x, y| x < y)
}

/// Checks if element equivalent to `ele` wrt comparator appears within range.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `rng.at(&i) < ele`.
///
/// # Postcondition
///   - Returns true if element equivalent to `ele` wrt is_less appears
///     within rng at `[start, end)`, otherwise returns false.
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
/// assert!(algo::binary_search_by(&arr, arr.start(), arr.end(), &4, |x, y| x < y));
/// ```
pub fn binary_search_by<Range, Compare>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    ele: &Range::Element,
    is_less: Compare,
) -> bool
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let i = lower_bound_by(rng, start, end.clone(), ele, is_less.clone());
    i != end && !is_less(ele, &rng.at(&i))
}

/// Checks if element equal to `ele` wrt comparator appears within range.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` should be partitioned wrt expression
///     `is_less(rng.at(&i), ele)`.
///
/// # Postcondition
///   - Returns true if element equal to `ele` appears
///     within rng at `[start, end)`, otherwise returns false.
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
/// assert!(algo::binary_search(&arr, arr.start(), arr.end(), &4));
/// ```
pub fn binary_search<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    ele: &Range::Element,
) -> bool
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    binary_search_by(rng, start, end, ele, |x, y| x < y)
}
