// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular, SemiRegular};

// Precondition:
//   - [start, end) represents valid position in rng.
//   - binary_pred should return true if 2 elements of rng are equivalent.
//   - binary_pred should follow equivalence relationship, otherwise behavior
//     is undefined.
// Postcondition:
//   - Removes all the adjacent equivalent elements from rng by moving them to
//     end of rng.
//   - NOTE: rng size would not be changed by this.
//   - Relative order of preserved elements are maintained.
//   - Relative order of removed elements are NOT guaranteed.
//   - Returns the position to new end of rng.
//   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
//
//   Where n is number of elements in [start, end).
pub fn unique_by<R, F>(
    rng: &mut R,
    mut start: R::Position,
    end: R::Position,
    bi_pred: F,
) -> R::Position
where
    R: OutputRange + ?Sized,
    F: Fn(&R::Element, &R::Element) -> bool,
{
    if start == end {
        return end;
    }
    let mut result = start.clone();
    start = rng.after(start);
    while start != end {
        if !bi_pred(rng.at(&result), rng.at(&start)) {
            result = rng.after(result);
            if result != start {
                rng.swap_at(&result, &start);
            }
        }
        start = rng.after(start);
    }
    rng.after(result)
}

// Precondition:
//   - [start, end) represents valid position in rng.
//   - Equality comparision should follow equivalence relationship otherwise
//     behavior is undefined.
// Postcondition:
//   - Removes all the adjacent equivalent elements from rng by moving them to
//     end of rng.
//   - NOTE: rng size would not be changed by this.
//   - Relative order of preserved elements are maintained.
//   - Relative order of removed elements are NOT guaranteed.
//   - Returns the position to new end of rng.
//   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
//
//   Where n is number of elements in [start, end).
pub fn unique<R>(
    rng: &mut R,
    start: R::Position,
    end: R::Position,
) -> R::Position
where
    R: OutputRange + ?Sized,
    R::Element: SemiRegular,
{
    unique_by(rng, start, end, |x, y| x == y)
}

// Precondition:
//   - [start, end) represents valid position in rng.
//   - dest can accomodate copied elements at out.
//   - binary_pred should return true if 2 elements of rng are equivalent.
//   - binary_pred should follow equivalence relationship, otherwise behavior
//     is undefined.
// Postcondition:
//   - Copies elements from rng from [start, end) with ommiting adjacent
//     equivalent elements by bi_pred to dest starting from out.
//   - Returns the position of past to last copied element in dest.
//   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
//
//   Where n is number of elements in [start, end).
pub fn unique_copy_by<R, D, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    dest: &mut D,
    mut out: D::Position,
    bi_pred: F,
) -> D::Position
where
    R: InputRange + ?Sized,
    D: OutputRange<Element = R::Element> + ?Sized,
    R::Element: Clone,
    F: Fn(&R::Element, &R::Element) -> bool,
{
    if start == end {
        return out;
    }
    *dest.at_mut(&out) = rng.at(&start).clone();
    start = rng.after(start);
    while start != end {
        if !bi_pred(dest.at(&out), rng.at(&start)) {
            out = dest.after(out);
            *dest.at_mut(&out) = rng.at(&start).clone();
        }
        start = rng.after(start);
    }
    dest.after(out)
}

// Precondition:
//   - [start, end) represents valid position in rng.
//   - dest can accomodate copied elements at out.
//   - R::Element equality comparision should follow equivalence relationsip
//     otherwise behavior is undefined.
// Postcondition:
//   - Copies elements from rng from [start, end) with ommiting adjacent
//     equivalent elements to dest starting from out.
//   - Returns the position of past to last copied element in dest.
//   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
//
//   Where n is number of elements in [start, end).
pub fn unique_copy<R, D>(
    rng: &R,
    start: R::Position,
    end: R::Position,
    dest: &mut D,
    out: D::Position,
) -> D::Position
where
    R: InputRange + ?Sized,
    D: OutputRange<Element = R::Element> + ?Sized,
    R::Element: Regular,
{
    unique_copy_by(rng, start, end, dest, out, |x, y| x == y)
}
