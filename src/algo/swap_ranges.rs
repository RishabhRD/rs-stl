// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::OutputRange;

/// Swaps elements of 2 ranges.
///
/// # Precondition
///   - `[start1, end1)` represents valid position in rng1.
///   - `[start2, end2)` represents valid position in rng2.
///
/// # Postcondition
///   - Swaps min(n1, n2) elements of rng1 at `[start1, end1)`
///     with elements of rng2 at `[start2, end2)`.
///   - Returns (position of last swapped element in rng1, position of last swapped element in rng2)
///   - Complexity: O(min(n1, n2)). Exactly min(n1, n2) swaps.
///
/// Where n1 is number of elements in `[start1, end1)` and n2 is number of
/// elements in `[start2, end2)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr1 = [1, 2];
/// let mut arr2 = [3, 4, 5];
/// let start1 = arr1.start();
/// let end1 = arr1.end();
/// let start2 = arr2.start();
/// let end2 = arr2.end();
/// let (i, j) = algo::swap_ranges(&mut arr1, start1, end1, &mut arr2, start2, end2);
/// assert_eq!(i, 2);
/// assert_eq!(j, 2);
/// assert!(arr1.equals(&[3, 4]));
/// assert!(arr2.equals(&[1, 2, 5]));
/// ```
pub fn swap_ranges<R1, R2>(
    rng1: &mut R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &mut R2,
    mut start2: R2::Position,
    end2: R2::Position,
) -> (R1::Position, R2::Position)
where
    R1: OutputRange,
    R2: OutputRange<Element = R1::Element>,
{
    while start1 != end1 && start2 != end2 {
        std::mem::swap(rng1.at_mut(&start1), rng2.at_mut(&start2));
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }

    (start1, start2)
}
