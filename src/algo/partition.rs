// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

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

/// Partitions range based on given predicate.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///
/// # Postcondition
///   - Reorders elements in rng at `[start, end)` such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is NOT preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n). Exactly n applications of pred. Atmost n swaps.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 3, 2, 5, 4];
/// let start = arr.start();
/// let end = arr.end();
/// let i = algo::partition(&mut arr, start, end, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].all_of(|x| x % 2 == 1));
/// assert!(arr[i..].all_of(|x| x % 2 == 0));
/// ```
pub fn partition<Range, Predicate>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> Range::Position
where
    Range: OutputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    while start != end {
        if !pred(rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }

    if start == end {
        return start;
    }

    let mut i = rng.after(start.clone());
    while i != end {
        if pred(rng.at(&i)) {
            rng.swap_at(&i, &start);
            start = rng.after(start);
        }
        i = rng.after(i);
    }

    start
}
