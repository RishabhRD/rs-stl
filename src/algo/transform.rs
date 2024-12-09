// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

// Precondition:
//   - [start, end) represents valid positions in rng.
//   - dest can accomodate transformed elements starting from out.
// Postcondition:
//   - Applies given function unary_op to elements of given range positions
//     and stores result in dest starting from out position.
//   - Complexity: O(n). Exactly n applications of unary_op. Where n is
//     number of elements in [start, end).
pub fn transform<R, D, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    dest: &mut D,
    mut out: D::Position,
    unary_op: F,
) -> D::Position
where
    R: InputRange,
    D: OutputRange,
    F: Fn(&R::Element) -> D::Element,
{
    while start != end {
        *dest.at_mut(&out) = unary_op(rng.at(&start));
        start = rng.after(start);
        out = dest.after(out);
    }
    out
}

// Precondition:
//   - [start1, end1) represents valid positions in rng1.
//   - [start2, start2 + n) represents valid positions in rng2.
//   - dest can accomodate transformed elements starting from out.
// Postcondition:
//   - Applies given function binary_op to elements of given range positions
//     and stores result in dest starting from out position.
//   - Complexity: O(n). Exactly n applications of binary_op. Where n is
//     number of elements in [start1, end1).
pub fn zip_transform<R1, R2, D, F>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    dest: &mut D,
    mut out: D::Position,
    binary_op: F,
) -> D::Position
where
    R1: InputRange,
    R2: InputRange,
    D: OutputRange,
    F: Fn(&R1::Element, &R2::Element) -> D::Element,
{
    while start1 != end1 {
        *dest.at_mut(&out) = binary_op(rng1.at(&start1), rng2.at(&start2));
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
        out = dest.after(out);
    }
    out
}
