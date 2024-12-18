// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular};

/// Precondition:
///   - `[start, end)` represents valid positions in rng.
///
/// Poscondition:
///   - Replaces element which satisfies pred of rng from
///     `[start, end)` with new_e.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn replace_if<R, F>(
    rng: &mut R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
    new_e: &R::Element,
) where
    R: OutputRange + ?Sized,
    R::Element: Clone,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            *rng.at_mut(&start) = new_e.clone();
        }
        start = rng.after(start);
    }
}

/// Precondition:
///   - `[start, end)` represents valid positions in rng.
///
/// Poscondition:
///   - Replaces all elements == old_e with new_e of rng from
///     `[start, end)`.
///   - Complexity: O(n). Exactly n equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
pub fn replace<R>(
    rng: &mut R,
    start: R::Position,
    end: R::Position,
    old_e: &R::Element,
    new_e: &R::Element,
) where
    R: OutputRange + ?Sized,
    R::Element: Regular,
{
    replace_if(rng, start, end, |x| x == old_e, new_e)
}

/// Precondition:
///   - `[start, end)` represents valid positions in rng.
///   - dest should be able to accomodate n elements starting from out.
///
/// Poscondition:
///   - Copies elements from `[start, end)` from rng to new range dest starting
///     from out while replacing all elements satisfying pred with new_e.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn replace_copy_if<R, D, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    dest: &mut D,
    mut out: D::Position,
    pred: F,
    new_e: &R::Element,
) -> D::Position
where
    R: InputRange + ?Sized,
    D: OutputRange<Element = R::Element> + ?Sized,
    R::Element: Clone,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            *dest.at_mut(&out) = new_e.clone();
        } else {
            *dest.at_mut(&out) = rng.at(&start).clone();
        }
        out = dest.after(out);
        start = rng.after(start);
    }
    out
}

/// Precondition:
///   - `[start, end)` represents valid positions in rng.
///   - dest should be able to accomodate elements being copied starting from out.
///
/// Poscondition:
///   - Copies elements from `[start, end)` from rng to new range dest starting
///     from out while replacing all elements == old_e with new_e.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn replace_copy<R, D>(
    rng: &R,
    start: R::Position,
    end: R::Position,
    dest: &mut D,
    out: D::Position,
    old_e: &R::Element,
    new_e: &R::Element,
) -> D::Position
where
    R: InputRange + ?Sized,
    D: OutputRange<Element = R::Element> + ?Sized,
    R::Element: Regular,
{
    replace_copy_if(rng, start, end, dest, out, |x| x == old_e, new_e)
}
