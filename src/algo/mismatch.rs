// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Precondition:
///   - [start1, end1) defines valid position of rng1
///   - [start2, start2 + n) defines valid positions of rng2 where n in number
///     of elements in [start1, end1)
///
/// Postcondition:
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches last1 or last2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` bi_pred applications.
pub fn mismatch_unbounded_by<R1, R2, F>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    bi_pred: F,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    F: Fn(&R1::Element, &R2::Element) -> bool,
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

/// Precondition:
///   - [start1, end1) defines valid position of rng1
///   - [start2, start2 + n) defines valid positions of rng2 where n in number
///     of elements in [start1, end1)
///
/// Postcondition:
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

/// Precondition:
///   - [start1, end1) defines valid position of rng1
///   - [start2, end2) defines valid positions of rng2
///
/// Postcondition:
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches last1 or last2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` bi_pred applications,
///     where n is min(#[start1, end1), #[start2, end2))
///     and # denotes number of elements
pub fn mismatch_by<R1, R2, F>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    end2: R2::Position,
    bi_pred: F,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    F: Fn(&R1::Element, &R2::Element) -> bool,
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

/// Precondition:
///   - [start1, end1) defines valid position of rng1
///   - [start2, start2 + n) defines valid positions of rng2 where n in number
///     of elements in [start1, end1)
///
/// Postcondition:
///   - Returns first position in rng1 and rng2 st rng1.at(p1) != rng2.at(p2)
///   - If no mismatches are found and comparision reaches last1 or last2
///     whichever happens first, returns positions of that point.
///   - Complexity: O(n), maximum `n` equality comparisions,
///     where n is min(#[start1, end1), #[start2, end2))
///     and # denotes number of elements
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
