// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, OutputRange};

use super::copy;

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

// Precondition:
//   - [start, mid) represent valid position in rng.
//   - [mid, end) represent valid position in rng.
//   - dest should be able to accomodate n elements starting from out.
// Postcondition:
//   - Copies elements from [start, end) of rng to dest starting from out in
//     such a way, that the element at mid becomes first element at out and
//     element at (mid - 1) becomes last element.
//   - Complexity: O(n). Exactly n assignments.
//
//   Where n is number of elements in [start, end).
pub fn rotate_copy<R, D>(
    rng: &R,
    start: R::Position,
    mid: R::Position,
    end: R::Position,
    dest: &mut D,
    mut out: D::Position,
) -> D::Position
where
    R: ForwardRange + ?Sized,
    R::Element: Clone,
    D: OutputRange<Element = R::Element> + ?Sized,
{
    out = copy(rng, mid.clone(), end, dest, out);
    copy(rng, start, mid, dest, out)
}
