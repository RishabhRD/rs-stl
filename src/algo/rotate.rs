// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::OutputRange;

// Precondition:
//   - [start, mid) represent valid position in rng.
//   - [mid, end) represent valid position in rng.
// Postcondition:
//   - Swaps element of [start, end) in such a way that the elements at
//     [start, mid) are placed after elements at [mid, end) while the orders
//     of both ranges are preserved.
//   - Returns position to element originally at first.
//   - Complexity: O(n). At most n swaps.
//
//   Where n is number of elements in [start, end).
//
// TODO: there are efficient implementations for BidirectionalRange and
// RandomAccessRange in rust. How to overload for them in rust?
pub fn rotate<R>(
    rng: &mut R,
    start: R::Position,
    mid: R::Position,
    end: R::Position,
) -> R::Position
where
    R: OutputRange + ?Sized,
{
    if start == mid {
        return end;
    }
    if mid == end {
        return start;
    }
    let mut write = start.clone();
    let mut next_read = start;
    let mut read = mid.clone();
    while read != end {
        if write == next_read {
            next_read = read.clone();
        }
        rng.swap_at(&write, &read);
        write = rng.after(write);
        read = rng.after(read);
    }
    rotate(rng, write.clone(), next_read, end);
    write
}
