// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

// Precondition:
//   - [start, end) represents valid position in `rng`.
// Postcondition:
//   - Returns true if all elements of position [start, end) in rng satisfies pred
//   - Otherwise, returns false.
//   - Complexity: O(n), maximum n invocations of `pred`. Where n is number
//     of elements in [start, end).
pub fn all_of<R, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
) -> bool
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if !pred(rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }
    true
}

// Precondition:
//   - [start, end) represents valid position in `rng`.
// Postcondition:
//   - Returns true if atleast one element of position [start, end) in rng satisfies pred
//   - Otherwise, returns false.
//   - Complexity: O(n), maximum n invocations of `pred`. Where n is number
//     of elements in [start, end).
pub fn any_of<R, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
) -> bool
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            return true;
        }
        start = rng.after(start);
    }
    false
}

// Precondition:
//   - [start, end) represents valid position in `rng`.
// Postcondition:
//   - Returns true if no element of position [start, end) in rng satisfies pred
//   - Otherwise, returns false.
//   - Complexity: O(n), maximum n invocations of `pred`. Where n is number
//     of elements in [start, end).
pub fn none_of<R, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
) -> bool
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }
    true
}
