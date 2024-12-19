// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, BidirectionalRange, OutputRange};

/// Reverses the given range.
///
/// # Precondition
///
/// # Postcondition
///   - Reverses the order of elements in rng.
///   - Complexity: O(n). Exactly (n / 2) swaps.
///
///   Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// rng::reverse(&mut arr);
/// assert!(arr.equals(&[3, 2, 1]));
///
/// let mut arr = [1, 2, 3];
/// arr.reverse();
/// assert!(arr.equals(&[3, 2, 1]));
/// ```
pub fn reverse<Range>(rng: &mut Range)
where
    Range: OutputRange + BidirectionalRange + ?Sized,
{
    algo::reverse(rng, rng.start(), rng.end())
}

pub mod infix {
    use crate::{rng, BidirectionalRange, OutputRange};

    /// `reverse`.
    pub trait STLReverseExt: OutputRange + BidirectionalRange {
        fn reverse(&mut self);
    }

    impl<R> STLReverseExt for R
    where
        R: OutputRange + BidirectionalRange + ?Sized,
    {
        fn reverse(&mut self) {
            rng::reverse(self)
        }
    }
}

/// Copies the given range in reverse order to dest.
///
/// # Precondition
///   - dest must be able to accomodate copied elements.
///
/// # Postcondition
///   - Copies elements from src in reverse order to dest.
///   - Returns position in dest after last copied element.
///   - Complexity: O(n). Exactly n assignments.
///
///   Where n is number of elements in src.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
/// let i = rng::reverse_copy(&src, &mut dest);
/// assert_eq!(i, 3);
/// assert!(dest.equals(&[3, 2, 1]));
/// ```
pub fn reverse_copy<SrcRange, DestRange>(
    rng: &SrcRange,
    dest: &mut DestRange,
) -> DestRange::Position
where
    SrcRange: BidirectionalRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
{
    algo::reverse_copy(rng, rng.start(), rng.end(), dest, dest.start())
}
