// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::OutputRange;

/// Precondition:
///   - `[start, end)` is represents valid positions in rng.
///
/// Postcondition:
///   - Fills element at `[start, end)` of rng with e.
///   - Complexity: O(n). Exactly n assignments.
///
///   Where n is number of elements in `[start, end)`.
pub fn fill_value<R>(
    rng: &mut R,
    mut start: R::Position,
    end: R::Position,
    e: &R::Element,
) where
    R: OutputRange + ?Sized,
    R::Element: Clone,
{
    while start != end {
        *rng.at_mut(&start) = e.clone();
        start = rng.after(start);
    }
}

/// Precondition:
///   - `[start, end)` is represents valid positions in rng.
///
/// Postcondition:
///   - Fills element at `[start, end)` of rng with e.
///   - Complexity: O(n). Exactly n application of gen.
///
///   Where n is number of elements in `[start, end)`.
pub fn fill_by<R, Gen>(
    rng: &mut R,
    mut start: R::Position,
    end: R::Position,
    gen: Gen,
) where
    R: OutputRange + ?Sized,
    Gen: Fn() -> R::Element,
{
    while start != end {
        *rng.at_mut(&start) = gen();
        start = rng.after(start);
    }
}
