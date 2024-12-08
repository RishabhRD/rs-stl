// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

// Precondition:
//   - [start1, end1) denotes valid positions in rng1.
//   - [start2, start2 + n) denotes valid positions in rng2 where n is number
//     of elements in [start1, end1).
// Postcondition:
//   - Returns true if range at [start1, end1) is equivalent to
//     range at [start2, start2 + n) by relationship `bi_pred`.
//   - Complexity: O(n). Maximum `n` bi_pred applications.
pub fn equals_unbounded_by<R1, R2, F>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    bi_pred: F,
) -> bool
where
    R1: InputRange,
    R2: InputRange,
    F: Fn(&R1::Element, &R2::Element) -> bool,
{
    while start1 != end1 {
        if !bi_pred(rng1.at(&start1), rng2.at(&start2)) {
            return false;
        }
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }
    true
}

// Precondition:
//   - [start1, end1) denotes valid positions in rng1.
//   - [start2, start2 + n) denotes valid positions in rng2 where n is number
//     of elements in [start1, end1).
// Postcondition:
//   - Returns true if range at [start1, end1) is equal to
//     range at [start2, start2 + n).
//   - Complexity: O(n). Maximum `n` equality comparisions.
pub fn equals_unbounded<R1, R2>(
    rng1: &R1,
    start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    start2: R2::Position,
) -> bool
where
    R1: InputRange,
    R2: InputRange,
    R1::Element: PartialEq<R2::Element>,
{
    equals_unbounded_by(rng1, start1, end1, rng2, start2, |x, y| x == y)
}

// Precondition:
//   - [start1, end1) denotes valid positions in rng1.
//   - [start2, end2) denotes valid positions in rng2
// Postcondition:
//   - Returns true if range at [start1, end1) is equivalent to
//     range at [start2, end2) by relationship bi_pred.
//   - Complexity: O(n). Maximum `n` `bi_pred` applications.
//     Where n is min(#[start1, end1), #[start2, end2)) and
//     # is number of elements in range operator.
pub fn equals_by<R1, R2, F>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    end2: R2::Position,
    bi_pred: F,
) -> bool
where
    R1: InputRange,
    R2: InputRange,
    F: Fn(&R1::Element, &R2::Element) -> bool,
{
    while start1 != end1 && start2 != end2 {
        if !bi_pred(rng1.at(&start1), rng2.at(&start2)) {
            return false;
        }
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }
    start1 == end1 && start2 == end2
}

// Precondition:
//   - [start1, end1) denotes valid positions in rng1.
//   - [start2, end2) denotes valid positions in rng2
// Postcondition:
//   - Returns true if range at [start1, end1) is equal to
//     range at [start2, end2).
//   - Complexity: O(n). Maximum `n` equality comparisions of elements.
//     Where n is min(#[start1, end1), #[start2, end2)) and
//     # is number of elements in range operator.
pub fn equals<R1, R2>(
    rng1: &R1,
    start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    start2: R2::Position,
    end2: R2::Position,
) -> bool
where
    R1: InputRange,
    R2: InputRange,
    R1::Element: PartialEq<R2::Element>,
{
    equals_by(rng1, start1, end1, rng2, start2, end2, |x, y| x == y)
}
