// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

/// Applies given operation on each element in range from left to right.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Applies `op` on each elements of rng at `[start, end)` from left to right.
///   - Complexity: O(n) applications of op.
///
/// Where n == `rng.distance(start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let mut sum = 0;
/// let arr = [1, 2, 3];
/// algo::for_each(&arr, arr.start(), arr.end(), |x| sum += *x);
/// assert_eq!(sum, 6);
/// ```
pub fn for_each<Range, UnaryOp>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    mut op: UnaryOp,
) where
    Range: InputRange + ?Sized,
    UnaryOp: FnMut(&Range::Element),
{
    while start != end {
        op(&rng.at(&start));
        start = rng.after(start);
    }
}

/// Applies given operation on each element in range from left to right.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Applies `op` on each elements of rng at `[start, end)` from left to right.
///   - Complexity: O(n) applications of op.
///
/// Where n == `rng.distance(start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let mut arr = [1, 2, 3];
/// let start = arr.start();
/// let end = arr.end();
/// algo::for_each_mut(&mut arr, start, end, |x| *x += 1);
/// assert_eq!(arr, [2, 3, 4]);
/// ```
pub fn for_each_mut<Range, UnaryOp>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    mut op: UnaryOp,
) where
    Range: OutputRange + ?Sized,
    UnaryOp: FnMut(&mut Range::Element),
{
    while start != end {
        op(&mut rng.at_mut(&start));
        start = rng.after(start);
    }
}
