// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::ForwardRange;

/// Returns position of mimimum element in range by comparator.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - cmp follows strict-weak-ordering.
///   - If a comes before b then cmp(a, b) == true otherwise false.
///
/// # Postcondition
///   - Returns position of minimum element in rng from `[start, end)` based
///     on comparator cmp. If there are multiple equivalent minimum elements,
///     then returns the position of first one of them.
///   - If `[start, end)` is empty then return end position.
///   - Complexity: O(n). Exactly max(n - 1, 0) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 1, 3, 1];
/// let i = algo::min_element_by(&arr, arr.start(), arr.end(), |x, y| x < y);
/// assert_eq!(i, 1);
/// ```
pub fn min_element_by<Range, Compare>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    cmp: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return start;
    }

    let mut smallest = start.clone();
    start = rng.after(start);

    while start != end {
        if cmp(rng.at(&start), rng.at(&smallest)) {
            smallest = start.clone();
        }
        start = rng.after(start);
    }

    smallest
}

/// Returns position of mimimum element in range.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Returns position minimum element in rng from `[start, end)`.
///     If there are multiple equal minimum elements, returns position of
///     first one of them.
///   - If `[start, end)` is empty then return end position.
///   - Complexity: O(n). Exactly max(n - 1, 0) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 1, 3, 1];
/// let i = algo::min_element(&arr, arr.start(), arr.end());
/// assert_eq!(i, 1);
/// ```
pub fn min_element<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    min_element_by(rng, start, end, |x, y| x < y)
}

/// Returns position of maximum element in the range by comparator.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///   - cmp should follow strict-weak-ordering.
///   - If a comes before b, then cmp(a, b) == true otherwise false.
///
/// # Postcondition
///   - Returns position of maximum element in rng `[start, end)`. If there
///     are multiple equivalent maximum element, then returns the position of
///     last one of them.
///   - Returns end if `[start, end)` is empty.
///   - Complexity: O(n). Exactly max(n - 1, 0) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 4, 3, 4, 2];
/// let i = algo::max_element_by(&arr, arr.start(), arr.end(), |x, y| x < y);
/// assert_eq!(i, 3);
/// ```
pub fn max_element_by<Range, Compare>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    cmp: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return end;
    }

    let mut max = start.clone();
    start = rng.after(start);

    while start != end {
        if !cmp(rng.at(&start), rng.at(&max)) {
            max = start.clone();
        }
        start = rng.after(start);
    }

    max
}

/// Returns position of maximum element in the range.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///
/// # Postcondition
///   - Returns position of maximum element in rng `[start, end)`. If there
///     are multiple equivalent maximum element, then returns the position of
///     last one of them.
///   - Returns end if `[start, end)` is empty.
///   - Complexity: O(n). Exactly max(n - 1, 0) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 4, 3, 4, 2];
/// let i = algo::max_element(&arr, arr.start(), arr.end());
/// assert_eq!(i, 3);
/// ```
pub fn max_element<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    max_element_by(rng, start, end, |x, y| x < y)
}
