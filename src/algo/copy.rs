// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

// Precondition:
//   - [start, end) represent valid positions in rng.
//   - dest should be able to accomodate all copied elements starting from out.
// Postcondition:
//   - copies elements from [start, end) satisfying `pred` to out position of
//     dest.
//   - Returns the position of dest just after last copy position.
//   - Complexity: O(n). Total n pred applications and maximum n copy operations.
//     where n is number of elements in [start, end) of rng.
pub fn copy_if<R, D, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    dest: &mut D,
    mut out: D::Position,
    pred: F,
) -> D::Position
where
    R: InputRange<Element = D::Element>,
    R::Element: Clone,
    D: OutputRange,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            *dest.at_mut(&out) = rng.at(&start).clone();
            out = dest.after(out);
        }
        start = rng.after(start);
    }
    out
}

// Precondition:
//   - [start, end) represent valid positions in rng.
//   - dest should be able to accomodate n elements starting from out where n
//     is number of elements in [start, end) of rng.
// Postcondition:
//   - copies elements from [start, end) to out position of dest.
//   - Returns the position of dest just after last copy position.
//   - Complexity: O(n). Total n copy operations.
pub fn copy<R, D>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    dest: &mut D,
    mut out: D::Position,
) -> D::Position
where
    R: InputRange<Element = D::Element>,
    R::Element: Clone,
    D: OutputRange,
{
    while start != end {
        *dest.at_mut(&out) = rng.at(&start).clone();
        out = dest.after(out);
        start = rng.after(start);
    }
    out
}

// Precondition:
//   - n >= 0.
//   - [start, start + n) represent valid positions in rng.
//   - dest should be able to accomodate n elements starting from out
// Postcondition:
//   - copies elements from [start, start + n) to out position of dest.
//   - Returns the position of dest just after last copy position.
//   - Complexity: O(n). Total n copy operations.
pub fn copy_n<R, D>(
    rng: &R,
    mut start: R::Position,
    mut n: usize,
    dest: &mut D,
    mut out: D::Position,
) -> D::Position
where
    R: InputRange<Element = D::Element>,
    R::Element: Clone,
    D: OutputRange,
{
    while n != 0 {
        *dest.at_mut(&out) = rng.at(&start).clone();
        out = dest.after(out);
        start = rng.after(start);
        n -= 1;
    }
    out
}
