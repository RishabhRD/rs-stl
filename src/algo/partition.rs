// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, InputRange, SemiOutputRange};

use super::rotate;

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
    Range: SemiOutputRange + ?Sized,
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

/// Partitions range based on given predicate with preserving relative order of elements.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///
/// # Postcondition
///   - Reorders elements in rng at `[start, end)` such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n.log2(n)). Exactly n applications of pred. Atmost n.log2(n) swaps.
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
/// let i = algo::stable_partition(&mut arr, start, end, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].equals(&[1, 3, 5]));
/// assert!(arr[i..].equals(&[2, 4]));
/// ```
pub fn stable_partition<Range, Predicate>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> Range::Position
where
    Range: SemiOutputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool + Clone,
{
    let n = rng.distance(start.clone(), end.clone());

    if n == 0 {
        return start;
    }
    if n == 1 {
        return if pred(rng.at(&start)) {
            rng.after(start)
        } else {
            start
        };
    }

    let mid = rng.after_n(start.clone(), n / 2);

    let left_start = stable_partition(rng, start, mid.clone(), pred.clone());
    let right_end = stable_partition(rng, mid.clone(), end, pred);
    rotate(rng, left_start, mid, right_end)
}

/// Returns the position of first such element in partitioned range such that predicate is not
/// satisfied.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` is partitioned based on pred.
///
/// # Postcondition
///   - Returns position of first element in rng at `[start, end)` such that
///     element at that position does not satisfy pred.
///   - If no such element exist, then returns end position.
///   - Complexity: O(log2(n)). log2(n) applications of pred. For traversal,
///     if range is RandomAccessRange then O(log2(n)) traversal otherwise
///     O(n.log2(n)) traversal.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 3, 5, 2, 4];
/// let i = algo::partition_point(&arr, arr.start(), arr.end(), |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// ```
pub fn partition_point<Range, Predicate>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> Range::Position
where
    Range: ForwardRange,
    Predicate: Fn(&Range::Element) -> bool,
{
    let mut n = rng.distance(start.clone(), end.clone());

    while n > 0 {
        let half = n / 2;
        let mid = rng.after_n(start.clone(), half);
        if pred(rng.at(&mid)) {
            start = rng.after(mid);
            n -= half + 1;
        } else {
            n = half;
        }
    }

    start
}
