// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

/// Applies given operation on each element in range from left to right.
///
/// # Precondition
///
/// # Postcondition
///   - Applies `op` on each elements of rng from left to right.
///   - Complexity: O(n) applications of op.
///
/// Where n == `rng.distance(rng.start(), rng.end())`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut sum = 0;
/// let arr = [1, 2, 3];
/// rng::for_each(&arr, |x| sum += *x);
/// assert_eq!(sum, 6);
///
/// let mut sum = 0;
/// let arr = [1, 2, 3];
/// arr.for_each(|x| sum += *x);
/// assert_eq!(sum, 6);
/// ```
pub fn for_each<Range, UnaryOp>(rng: &Range, mut op: UnaryOp)
where
    Range: InputRange + ?Sized,
    UnaryOp: FnMut(&Range::Element),
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        op(rng.at(&start));
        start = rng.after(start);
    }
}

/// Applies given operation on each element in range from left to right.
///
/// # Precondition
///
/// # Postcondition
///   - Applies `op` on each elements of rng from left to right.
///   - Complexity: O(n) applications of op.
///
/// Where n == `rng.distance(rng.start(), rng.end())`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// rng::for_each_mut(&mut arr, |x| *x += 1);
/// assert_eq!(arr, [2, 3, 4]);
///
/// let mut arr = [1, 2, 3];
/// arr.for_each_mut(|x| *x += 1);
/// assert_eq!(arr, [2, 3, 4]);
/// ```
pub fn for_each_mut<Range, UnaryOp>(rng: &mut Range, mut op: UnaryOp)
where
    Range: OutputRange + ?Sized,
    UnaryOp: FnMut(&mut Range::Element),
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        op(rng.at_mut(&start));
        start = rng.after(start);
    }
}

pub mod infix {
    use crate::{rng, InputRange, OutputRange};

    /// `for_each`.
    pub trait STLForEachInputExt: InputRange {
        fn for_each<UnaryOp>(&self, op: UnaryOp)
        where
            UnaryOp: FnMut(&Self::Element),
        {
            rng::for_each(self, op)
        }
    }

    impl<R> STLForEachInputExt for R where R: InputRange + ?Sized {}

    /// `for_each_mut`.
    pub trait STLForEachOutputExt: OutputRange {
        fn for_each_mut<UnaryOp>(&mut self, op: UnaryOp)
        where
            UnaryOp: FnMut(&mut Self::Element),
        {
            rng::for_each_mut(self, op)
        }
    }
    impl<R> STLForEachOutputExt for R where R: OutputRange + ?Sized {}
}
