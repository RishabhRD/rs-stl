// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Finds first mismatch position between 2 ranges by given equivalence relation where second range
/// is considered to be atleast as big as first one.
///
/// # Precondition
///   - `[start1, end1)` defines valid position of rng1
///   - `[start2, start2 + n)` defines valid positions of rng2 where n in number
///     of elements in `[start1, end1)`
///   - bi_pred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches last1 or last2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` bi_pred applications.
pub fn mismatch_unbounded_by<R1, R2, BinaryPred>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    bi_pred: BinaryPred,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    BinaryPred: Fn(&R1::Element, &R2::Element) -> bool,
{
    while start1 != end1 {
        if !bi_pred(rng1.at(&start1), rng2.at(&start2)) {
            return (start1, start2);
        }
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }
    (start1, start2)
}

/// Finds first mismatch position between 2 ranges by equality where second range
/// is considered to be atleast as big as first one.
///
/// # Precondition
///   - `[start1, end1)` defines valid position of rng1
///   - `[start2, start2 + n)` defines valid positions of rng2 where n in number
///     of elements in `[start1, end1)`
///   - bi_pred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches last1 or last2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` equality comparisions.
pub fn mismatch_unbounded<R1, R2>(
    rng1: &R1,
    start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    start2: R2::Position,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    mismatch_unbounded_by(rng1, start1, end1, rng2, start2, |x, y| x == y)
}

/// Finds first mismatch position between 2 ranges by given equivalence relation.
///
/// # Precondition
///   - `[start1, end1)` defines valid position of rng1
///   - `[start2, end2)` defines valid positions of rng2
///   - bi_pred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches end1 or end2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` bi_pred applications.
///
///     where n is min(#`[start1, end1)`, #`[start2, end2)`)
///     and # denotes number of elements
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 4];
///
/// let (i, j) = algo::mismatch_by(&arr1, arr1.start(), arr1.end(), &arr2, arr2.start(), arr2.end(), |x, y| x == y);
/// assert_eq!(i, 2);
/// assert_eq!(j, 2);
/// ```
pub fn mismatch_by<R1, R2, BinaryPred>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    end2: R2::Position,
    bi_pred: BinaryPred,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    BinaryPred: Fn(&R1::Element, &R2::Element) -> bool,
{
    while start1 != end1 && start2 != end2 {
        if !bi_pred(rng1.at(&start1), rng2.at(&start2)) {
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
///   - `[start1, end1)` defines valid position of rng1
///   - `[start2, end2)` defines valid positions of rng2
///
/// # Postcondition
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches end1 or end2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` equality comparisions.
///
///     where n is min(#`[start1, end1)`, #`[start2, end2)`)
///     and # denotes number of elements
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 4];
///
/// let (i, j) = algo::mismatch(&arr1, arr1.start(), arr1.end(), &arr2, arr2.start(), arr2.end());
/// assert_eq!(i, 2);
/// assert_eq!(j, 2);
/// ```
pub fn mismatch<R1, R2>(
    rng1: &R1,
    start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    start2: R2::Position,
    end2: R2::Position,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    mismatch_by(rng1, start1, end1, rng2, start2, end2, |x, y| x == y)
}
