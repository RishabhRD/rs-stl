// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, OutputRange};

/// Swaps elements of 2 ranges.
///
/// # Precondition
///
/// # Postcondition
///   - Swaps min(n1, n2) elements of rng1 with elements of rng2.
///   - Returns (position of last swapped element in rng1, position of last swapped element in rng2)
///   - Complexity: O(min(n1, n2)). Exactly min(n1, n2) swaps.
///
/// Where n1 is number of elements in rng1 and n2 is number of elements in rng2.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr1 = [1, 2];
/// let mut arr2 = [3, 4, 5];
/// let (i, j) = rng::swap_ranges(&mut arr1, &mut arr2);
/// assert_eq!(i, 2);
/// assert_eq!(j, 2);
/// assert!(arr1.equals(&[3, 4]));
/// assert!(arr2.equals(&[1, 2, 5]));
/// ```
pub fn swap_ranges<R1, R2>(
    rng1: &mut R1,
    rng2: &mut R2,
) -> (R1::Position, R2::Position)
where
    R1: OutputRange + ?Sized,
    R2: OutputRange<Element = R1::Element> + ?Sized,
{
    let mut start1 = rng1.start();
    let mut start2 = rng2.start();

    while !rng1.is_end(&start1) && !rng2.is_end(&start2) {
        std::mem::swap(rng1.at_mut(&start1), rng2.at_mut(&start2));
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }

    (start1, start2)
}
