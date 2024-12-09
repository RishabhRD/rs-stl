// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OutputRange, Regular};

// Precondition:
//   - [start, end) represents valid positions in rng.
// Poscondition:
//   - Replaces element which satisfies pred of rng from
//     [start, end) with new_e.
//   - Complexity: O(n). Exactly n applications of pred.
//
//   Where n is number of elements in [start, end).
pub fn replace_if<R, F>(
    rng: &mut R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
    new_e: &R::Element,
) where
    R: OutputRange,
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

// Precondition:
//   - [start, end) represents valid positions in rng.
// Poscondition:
//   - Replaces all elements == old_e with new_e of rng from
//     [start, end).
//   - Complexity: O(n). Exactly n equality comparisions.
//
//   Where n is number of elements in [start, end).
pub fn replace<R>(
    rng: &mut R,
    start: R::Position,
    end: R::Position,
    old_e: &R::Element,
    new_e: &R::Element,
) where
    R: OutputRange,
    R::Element: Regular,
{
    replace_if(rng, start, end, |x| x == old_e, new_e)
}
