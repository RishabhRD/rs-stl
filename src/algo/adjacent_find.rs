// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::ForwardRange;

// Precondition:
//   - [start, end) denotes valid positions in rng
// Postcondition:
//   - Returns first position in [start, end) of rng such that element at that
//     position and element after that position satisfies bi_pred.
//   - Returns end if no such element is found
pub fn adjacent_find_if<R, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    bi_pred: F,
) -> R::Position
where
    R: ForwardRange,
    F: Fn(&R::Element, &R::Element) -> bool,
{
    if start == end {
        return end;
    }
    let mut prev = start.clone();
    start = rng.after(start);
    while start != end {
        if bi_pred(rng.at(&prev), rng.at(&start)) {
            return start;
        }
        prev = start.clone();
        start = rng.after(start);
    }
    end
}
