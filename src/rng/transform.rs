// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange, OutputRange};

pub fn transform<R, D, F>(rng: &R, dest: &mut D, unary_op: F) -> D::Position
where
    R: InputRange + ?Sized,
    D: OutputRange + ?Sized,
    F: Fn(&R::Element) -> D::Element,
{
    algo::transform(rng, rng.start(), rng.end(), dest, dest.start(), unary_op)
}

/// Precondition:
///   - dest can accomodate n transformed elements starting from out.
///
/// Postcondition:
///   - Applies given function binary_op to elements of given range positions
///     and stores result in dest starting from out position.
///   - Complexity: O(n). Exactly n applications of binary_op.
///
///   Where n is minimum of number of elements in rng1, rng2.
pub fn zip_transform<R1, R2, D, F>(
    rng1: &R1,
    rng2: &R2,
    dest: &mut D,
    binary_op: F,
) -> D::Position
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    D: OutputRange + ?Sized,
    F: Fn(&R1::Element, &R2::Element) -> D::Element,
{
    let mut start1 = rng1.start();
    let end1 = rng1.end();
    let mut start2 = rng2.start();
    let end2 = rng2.end();
    let mut out = dest.start();
    while start1 != end1 && start2 != end2 {
        *dest.at_mut(&out) = binary_op(rng1.at(&start1), rng2.at(&start2));
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
        out = dest.after(out);
    }
    out
}
