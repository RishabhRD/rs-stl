// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, OutputRange};

/// Reverses the given range.
///
/// Precondition:
///   - `[start, end)` represents valid positions in rng.
///
/// Postcondition:
///   - Reverses the order of elements at `[start, end)` in rng.
///   - Complexity: O(n). Exactly (n / 2) swaps.
///
///   Where n is number of elements in `[start, end)` of rng.
pub fn reverse<Range>(
    rng: &mut Range,
    mut start: Range::Position,
    mut end: Range::Position,
) where
    Range: OutputRange + BidirectionalRange + ?Sized,
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

/// Copies the given range in reverse order to dest.
///
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
pub fn reverse_copy<SrcRange, DestRange>(
    rng: &SrcRange,
    start: SrcRange::Position,
    mut end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
) -> DestRange::Position
where
    SrcRange: BidirectionalRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
{
    while start != end {
        end = rng.before(end);
        *dest.at_mut(&out) = rng.at(&end).clone();
        out = dest.after(out);
    }
    out
}
