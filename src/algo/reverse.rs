// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, OutputRange};

// Precondition:
//   - [start, end) represents valid positions in rng.
// Postcondition:
//   - Reverses the order of elements at [start, end) in rng.
//   - Complexity: O(n). Exactly (n / 2) swaps.
//
//   Where n is number of elements in [start, end) of rng.
pub fn reverse<R>(rng: &mut R, mut start: R::Position, mut end: R::Position)
where
    R: OutputRange + BidirectionalRange + ?Sized,
{
    while start != end {
        end = rng.before(end);
        if start == end {
            break;
        }
        rng.swap_at(&start, &end);
        start = rng.after(start);
    }
}
