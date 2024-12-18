// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::OutputRange;

/// Fills rng `[start, end)` with given value.
///
/// Precondition:
///   - `[start, end)` is represents valid positions in rng.
///
/// Postcondition:
///   - Fills element at `[start, end)` of rng with e.
///   - Complexity: O(n). Exactly n assignments.
///
///   Where n is number of elements in `[start, end)`.
pub fn fill_value<Range>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    e: &Range::Element,
) where
    Range: OutputRange + ?Sized,
    Range::Element: Clone,
{
    while start != end {
        *rng.at_mut(&start) = e.clone();
        start = rng.after(start);
    }
}

/// Fills rng `[start, end)` using given generator.
///
/// Precondition:
///   - `[start, end)` is represents valid positions in rng.
///
/// Postcondition:
///   - Fills element at `[start, end)` of rng with e.
///   - Complexity: O(n). Exactly n application of gen.
///
///   Where n is number of elements in `[start, end)`.
pub fn fill_by<Range, Gen>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    gen: Gen,
) where
    Range: OutputRange + ?Sized,
    Gen: Fn() -> Range::Element,
{
    while start != end {
        *rng.at_mut(&start) = gen();
        start = rng.after(start);
    }
}
