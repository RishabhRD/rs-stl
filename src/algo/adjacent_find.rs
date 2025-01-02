// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::ForwardRange;

/// Returns first position in range such that element at that
/// position and element after that position satisfies binary predicate.
///
/// # Precondition
///   - `[start, end)` denotes valid positions in rng
///
/// # Postcondition
///   - Returns first position in `[start, end)` of rng such that element at that
///     position and element after that position satisfies bi_pred.
///   - Returns end if no such element is found
///   - Complexity: O(n), maximum `n - 1` calls to `bi_pred`.
///     Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 1, 3];
///
/// let i = algo::adjacent_find_if(&arr, arr.start(), arr.end(), |x, y| x == y);
/// assert_eq!(i, 0);
/// ```
pub fn adjacent_find_if<Range, BinaryPred>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    bi_pred: BinaryPred,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    BinaryPred: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return end;
    }
    let mut prev = start.clone();
    start = rng.after(start);
    while start != end {
        if bi_pred(&rng.at(&prev), &rng.at(&start)) {
            return prev;
        }
        prev = start.clone();
        start = rng.after(start);
    }
    end
}
