// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

/// Copies elements of src to dest with applying unary operation on it.
///
/// # Precondition
///   - dest can accomodate transformed elements.
///
/// # Postcondition
///   - Applies given function unary_op to elements of given range positions
///     and stores result in dest.
///   - Complexity: O(n). Exactly n applications of unary_op.
///
///   Where n is number of elements in src.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let input = [1, 2, 3];
/// let mut out = [0, 0, 0, 0];
///
/// let j = rng::transform(&input, &mut out, |x| x + 1);
/// assert!(out.equals(&[2, 3, 4, 0]));
/// assert_eq!(j, 3);
/// ```
pub fn transform<SrcRange, DestRange, UnaryOp>(
    src: &SrcRange,
    dest: &mut DestRange,
    unary_op: UnaryOp,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange + ?Sized,
    UnaryOp: Fn(&SrcRange::Element) -> DestRange::Element,
{
    let mut start = src.start();
    let mut out = dest.start();
    while !src.is_end(&start) {
        *dest.at_mut(&out) = unary_op(&src.at(&start));
        start = src.after(start);
        out = dest.after(out);
    }
    out
}

/// Stores result in dest after applying binary operation on zipped elements of given ranges.
///
/// # Precondition
///   - dest can accomodate n transformed elements.
///
/// # Postcondition
///   - Applies given function binary_op to elements of given range positions
///     and stores result in dest.
///   - Complexity: O(n). Exactly n applications of binary_op.
///
///   Where n is minimum of number of elements in rng1, rng2.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let input = vec![1, 2, 3];
/// let input1 = vec![1, 2];
/// let mut out = vec![0, 0, 0, 0];
/// let j = rng::zip_transform(&input, &input1, &mut out, |x, y| x * y);
/// assert!(out.equals(&vec![1, 4, 0, 0]));
/// assert_eq!(j, 2);
/// ```
pub fn zip_transform<R1, R2, DestRange, BinaryOp>(
    rng1: &R1,
    rng2: &R2,
    dest: &mut DestRange,
    binary_op: BinaryOp,
) -> DestRange::Position
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    DestRange: OutputRange + ?Sized,
    BinaryOp: Fn(&R1::Element, &R2::Element) -> DestRange::Element,
{
    let mut start1 = rng1.start();
    let mut start2 = rng2.start();
    let mut out = dest.start();
    while !rng1.is_end(&start1) && !rng2.is_end(&start2) {
        *dest.at_mut(&out) = binary_op(&rng1.at(&start1), &rng2.at(&start2));
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
        out = dest.after(out);
    }
    out
}
