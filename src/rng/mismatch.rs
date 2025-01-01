// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Finds first mismatch position between 2 ranges by given equivalence relation.
///
/// # Precondition
///   - bi_pred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches end1 or end2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` bi_pred applications,
///
///     where n is min(#rng1, #rng2) and # denotes number of elements in range.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 4];
///
/// let (i, j) = rng::mismatch_by(&arr1, &arr2, |x, y| x == y);
/// assert_eq!(i, 2);
/// assert_eq!(j, 2);
/// ```
pub fn mismatch_by<R1, R2, BinaryPred>(
    rng1: &R1,
    rng2: &R2,
    bi_pred: BinaryPred,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    BinaryPred: Fn(&R1::Element, &R2::Element) -> bool,
{
    let mut start1 = rng1.start();
    let mut start2 = rng2.start();

    while !rng1.is_end(&start1) && !rng2.is_end(&start2) {
        if !bi_pred(&rng1.at(&start1), &rng2.at(&start2)) {
            return (start1, start2);
        }
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }
    (start1, start2)
}

/// Finds first mismatch position between 2 ranges by equality.
///
/// # Precondition
///
/// # Postcondition
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches end1 or end2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` equality comparisions.
///
///     where n is min(#rng1, #rng2) and # denotes number of elements in range.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 4];
///
/// let (i, j) = rng::mismatch(&arr1, &arr2);
/// assert_eq!(i, 2);
/// assert_eq!(j, 2);
/// ```
pub fn mismatch<R1, R2>(rng1: &R1, rng2: &R2) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    mismatch_by(rng1, rng2, |x, y| x == y)
}
