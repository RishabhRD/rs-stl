// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, OutputRange};

/// Precondition:
///   - `[start, end)` represents valid positions in rng.
///
/// Postcondition:
///   - Reverses the order of elements at `[start, end)` in rng.
///   - Complexity: O(n). Exactly (n / 2) swaps.
///
///   Where n is number of elements in `[start, end)` of rng.
pub fn reverse<R>(rng: &mut R, mut start: R::Position, mut end: R::Position)
where
    R: OutputRange + BidirectionalRange + ?Sized,
{
    while start != end {
        end = rng.before(end);
        if start == end {
            break;
        }
        rng.swap_at(&start, &end);
        start = rng.after(start);
    }
}

/// Precondition:
///   - `[start, end)` represents valid positions in rng.
///   - dest must be able to accomodate copied elements starting from out.
///
/// Postcondition:
///   - Copies elements from `[start, end)` of rng in reverse order to dest
///     starting from out.
///   - Complexity: O(n). Exactly n assignments.
///
///   Where n is number of elements in `[start, end)` of rng.
pub fn reverse_copy<R, D>(
    rng: &R,
    start: R::Position,
    mut end: R::Position,
    dest: &mut D,
    mut out: D::Position,
) -> D::Position
where
    R: BidirectionalRange + ?Sized,
    D: OutputRange<Element = R::Element> + ?Sized,
    R::Element: Clone,
{
    while start != end {
        end = rng.before(end);
        *dest.at_mut(&out) = rng.at(&end).clone();
        out = dest.after(out);
    }
    out
}
