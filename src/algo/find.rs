// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

// Precondition:
//   - [start, end) represent valid positions in rng.
// Postcondition:
//   - Returns position of first element in [start, end) of rng satisfying pred.
//   - Returns end if no such element exists.
//   - Complexity: O(n) where n is number if elements in [start, end).
pub fn find_if<R, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
) -> R::Position
where
    R: InputRange,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            break;
        }
        start = rng.after(start)
    }
    start
}
