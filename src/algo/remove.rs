// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular};

/// Precondition:
///   - \`[start, end)` represents valid positions in rng
///
/// Postcondition:
///   - Removes element satisfying pred by moving them to end of rng.
///   - NOTE: remove_if doesn't resize rng.
///   - Relative ordering of elements NOT satisfying pred is preserved.
///   - Relative ordering of removed elements is NOT guaranteed.
///   - Returns end position for preserved elements in rng.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove_if<R, F>(
    rng: &mut R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
) -> R::Position
where
    R: OutputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }
    if start != end {
        let mut i = rng.after(start.clone());
        while i != end {
            if !pred(rng.at(&i)) {
                rng.swap_at(&i, &start);
                start = rng.after(start);
            }
            i = rng.after(i);
        }
    }

    start
}

/// Precondition:
///   - `[start, end)` represents valid positions in rng
///
/// Postcondition:
///   - Removes element == val by moving them to end of rng.
///   - NOTE: remove doesn't resize rng.
///   - Relative ordering of preserved elements are maintained.
///   - Relative ordering of removed elements is NOT guaranteed.
///   - Returns end position for preserved elements in rng.
///   - Complexity: O(n). Exactly n equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove<R>(
    rng: &mut R,
    start: R::Position,
    end: R::Position,
    val: &R::Element,
) -> R::Position
where
    R: OutputRange + ?Sized,
    R::Element: Eq,
{
    remove_if(rng, start, end, |x| x == val)
}

/// Precondition:
///   - `[start, end)` represents valid positions in rng
///   - dest can accomodate copied elements starting from out.
///
/// Postcondition:
///   - Copies elements from `[start, end)` of `rng` to starting from `out`
///     of `dest`, ommiting the elements which satisfies pred.
///   - Returns position of past the last element copied in dest.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove_copy_if<R, D, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    dest: &mut D,
    mut out: D::Position,
    pred: F,
) -> D::Position
where
    R: InputRange + ?Sized,
    R::Element: Clone,
    D: OutputRange<Element = R::Element> + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if !pred(rng.at(&start)) {
            *dest.at_mut(&out) = rng.at(&start).clone();
            out = dest.after(out);
        }
        start = rng.after(start);
    }
    out
}

/// Precondition:
///   - `[start, end)` represents valid positions in rng
///   - dest can accomodate copied elements starting from out.
///
/// Postcondition:
///   - Copies elements from `[start, end)` of `rng` to starting from `out`
///     of `dest`, ommiting the elements == val.
///   - Returns position of past the last element copied in dest.
///   - Complexity: O(n). Exactly n equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove_copy<R, D>(
    rng: &R,
    start: R::Position,
    end: R::Position,
    dest: &mut D,
    out: D::Position,
    val: &R::Element,
) -> D::Position
where
    R: InputRange + ?Sized,
    R::Element: Regular,
    D: OutputRange<Element = R::Element> + ?Sized,
{
    remove_copy_if(rng, start, end, dest, out, |x| x == val)
}
