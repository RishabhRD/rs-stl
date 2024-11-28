// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

// Precondition:
//   - [start, end) represent valid positions in rng.
// Postcondition:
//   - Returns position of first element in [start, end) of rng satisfying pred.
//   - Returns end if no such element exists.
//   - Complexity: O(n) where n is number if elements in [start, end).
pub fn find_if<T, F>(
    rng: &T,
    mut start: T::Position,
    end: T::Position,
    pred: F,
) -> T::Position
where
    T: InputRange,
    F: Fn(&T::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            break;
        }
        start = rng.position_after(start)
    }
    start
}
