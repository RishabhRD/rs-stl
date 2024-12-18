// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Returns true if all elements in range satisfies the predicate.
///
/// # Precondition
///   - `[start, end)` represents valid position in `rng`.
///
/// # Postcondition
///   - Returns true if all elements of position `[start, end)` in rng satisfies pred
///   - Otherwise, returns false.
///   - Complexity: O(n), maximum n invocations of `pred`. Where n is number
///     of elements in `[start, end)`.
pub fn all_of<Range, Pred>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Pred,
) -> bool
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    while start != end {
        if !pred(rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }
    true
}

/// Returns true if atleast one element in range satisfies the predicate.
///
/// # Precondition
///   - `[start, end)` represents valid position in `rng`.
///
/// # Postcondition
///   - Returns true if atleast one element of position `[start, end)` in rng satisfies pred
///   - Otherwise, returns false.
///   - Complexity: O(n), maximum n invocations of `pred`. Where n is number
///     of elements in `[start, end)`.
pub fn any_of<Range, Pred>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Pred,
) -> bool
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            return true;
        }
        start = rng.after(start);
    }
    false
}

/// Returns true if no element in range satisfies the predicate.
///
/// # Precondition
///   - `[start, end)` represents valid position in `rng`.
///
/// # Postcondition
///   - Returns true if no element of position `[start, end)` in rng satisfies pred
///   - Otherwise, returns false.
///   - Complexity: O(n), maximum n invocations of `pred`. Where n is number
///     of elements in `[start, end)`.
pub fn none_of<Range, Pred>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Pred,
) -> bool
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }
    true
}
