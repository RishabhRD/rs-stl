// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OutputRange, View};

/// Fills rng with given value.
///
/// # Precondition
///
/// # Postcondition
///   - Fills element in rng with value e.
///   - Complexity: O(n). Exactly n assignments.
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
/// let mut arr = [0, 0, 0];
/// rng::fill_value(&mut arr, &2);
/// assert!(arr.equals(&[2, 2, 2]));
///
/// let mut arr = [0, 0, 0];
/// arr.fill_value(&2);
/// assert!(arr.equals(&[2, 2, 2]));
/// ```
pub fn fill<Range>(rng: &mut Range, e: &Range::Element)
where
    Range: View + OutputRange + ?Sized,
    Range::Element: Clone,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        *rng.at_mut(&start) = e.clone();
        start = rng.after(start);
    }
}

/// Fills rng using given generator.
///
/// # Precondition
///
/// # Postcondition
///   - Fills element in rng with e with result of invoking gen.
///   - Complexity: O(n). Exactly n application of gen.
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
/// let mut arr = [0, 0, 0];
/// rng::fill_by(&mut arr, || 2);
/// assert!(arr.equals(&[2, 2, 2]));
///
/// let mut arr = [0, 0, 0];
/// arr.fill_by(|| 2);
/// assert!(arr.equals(&[2, 2, 2]));
/// ```
pub fn fill_by<Range, Gen>(rng: &mut Range, gen: Gen)
where
    Range: View + OutputRange + ?Sized,
    Gen: Fn() -> Range::Element,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        *rng.at_mut(&start) = gen();
        start = rng.after(start);
    }
}

pub mod infix {
    use crate::{rng, OutputRange, View};

    /// `fill`, `fill_by`.
    pub trait STLFillExt: View + OutputRange {
        fn fill(&mut self, e: &Self::Element)
        where
            Self::Element: Clone,
        {
            rng::fill(self, e)
        }

        fn fill_by<Gen>(&mut self, gen: Gen)
        where
            Self::Element: Clone,
            Gen: Fn() -> Self::Element,
        {
            rng::fill_by(self, gen)
        }
    }

    impl<R> STLFillExt for R where R: OutputRange + View + ?Sized {}
}
