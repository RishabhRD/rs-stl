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
///   - Complexity: O(n), maximum n invocations of `pred`.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 3, 5];
/// assert!(algo::all_of(&arr, arr.start(), arr.end(), |x| x % 2 == 1));
/// ```
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
///   - Complexity: O(n), maximum n invocations of `pred`.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 4, 5];
/// assert!(algo::any_of(&arr, arr.start(), arr.end(), |x| x % 2 == 1));
/// ```
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
///   - Complexity: O(n), maximum n invocations of `pred`.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [2, 4, 6];
/// assert!(algo::none_of(&arr, arr.start(), arr.end(), |x| x % 2 == 1));
/// ```
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
