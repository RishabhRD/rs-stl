// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Returns true if range is partitioned wrt pred, otherwise false.
///
/// # Precondition
///   - `[start, end)` reperesents valid position in rng.
///
/// # Postcondition
///   - Returns true if rng at `[start, end)` is partitioned wrt pred. i.e.,
///     There should be NO position `i` and `j` in `[start, end)` such that
///     i comes before j and
///     `pred(rng.at(&i)) == false && pred(rng.at(&j)) == true`.
///   - Otherwise, returns false.
///   - Complexity: O(n). At most n applications of pred.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 3, 5, 2, 4];
/// assert!(algo::is_partitioned(&arr, arr.start(), arr.end(), |x| x % 2 == 1));
/// ```
pub fn is_partitioned<Range, Predicate>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> bool
where
    Range: InputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    while start != end {
        if !pred(rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }

    while start != end {
        if pred(rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }

    true
}
