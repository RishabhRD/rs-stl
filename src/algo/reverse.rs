// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, OutputRange, SemiOutputRange};

/// Reverses the given range.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Reverses the order of elements at `[start, end)` in rng.
///   - Complexity: O(n). Exactly (n / 2) swaps.
///
///   Where n is number of elements in `[start, end)` of rng.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// let start = arr.start();
/// let end = arr.end();
/// algo::reverse(&mut arr, start, end);
/// assert!(arr.equals(&[3, 2, 1]));
/// ```
pub fn reverse<Range>(
    rng: &mut Range,
    mut start: Range::Position,
    mut end: Range::Position,
) where
    Range: SemiOutputRange + BidirectionalRange + ?Sized,
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
/// # Precondition
///   - `[start, end)` represents valid positions src.
///   - dest must be able to accomodate copied elements starting from out.
///
/// # Postcondition
///   - Copies elements from `[start, end)` of src in reverse order to dest
///     starting from out.
///   - Returns position in dest after last copied element.
///   - Complexity: O(n). Exactly n assignments.
///
///   Where n is number of elements in `[start, end)` of src.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
/// let out = dest.start();
/// let i = algo::reverse_copy(&src, src.start(), src.end(), &mut dest, out);
/// assert_eq!(i, 3);
/// assert!(dest.equals(&[3, 2, 1]));
/// ```
pub fn reverse_copy<SrcRange, DestRange>(
    src: &SrcRange,
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
        end = src.before(end);
        *dest.at_mut(&out) = src.at(&end).clone();
        out = dest.after(out);
    }
    out
}
