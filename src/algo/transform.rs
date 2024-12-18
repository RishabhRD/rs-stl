// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

/// Copies elements of src to dest with applying unary operation on it.
///
/// # Precondition
///   - `[start, end)` represents valid positions in src.
///   - dest can accomodate transformed elements starting from out.
///
/// # Postcondition
///   - Applies given function unary_op to elements of given range positions
///     and stores result in dest starting from out position.
///   - Complexity: O(n). Exactly n applications of unary_op.
///
///   Where n is number of elements in `[start, end)`.
pub fn transform<SrcRange, DestRange, UnaryOp>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    unary_op: UnaryOp,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange + ?Sized,
    UnaryOp: Fn(&SrcRange::Element) -> DestRange::Element,
{
    while start != end {
        *dest.at_mut(&out) = unary_op(src.at(&start));
        start = src.after(start);
        out = dest.after(out);
    }
    out
}

/// Copies elements of rng1 and rng2 to dest with applying binary operation on it.
///
/// # Precondition
///   - `[start1, end1)` represents valid positions in rng1.
///   - `[start2, start2 + n)` represents valid positions in rng2.
///   - dest can accomodate transformed elements starting from out.
///
/// # Postcondition
///   - Applies given function binary_op to elements of given range positions
///     and stores result in dest starting from out position.
///   - Complexity: O(n). Exactly n applications of binary_op. Where n is
///     number of elements in `[start1, end1)`.
pub fn zip_transform<R1, R2, DestRange, BinaryOp>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    binary_op: BinaryOp,
) -> DestRange::Position
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    DestRange: OutputRange + ?Sized,
    BinaryOp: Fn(&R1::Element, &R2::Element) -> DestRange::Element,
{
    while start1 != end1 {
        *dest.at_mut(&out) = binary_op(rng1.at(&start1), rng2.at(&start2));
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
        out = dest.after(out);
    }
    out
}
