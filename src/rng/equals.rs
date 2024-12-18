// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

/// Returns true if rng1 elements are equivalent to elements of rng2 starting from start2 by relation bi_pred.
///
/// # Precondition
///   - `[start2, start2 + n)` denotes valid positions in rng2
///   - BinaryPred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns true if rng1 is equivalent to rng2 at `[start2, start2 + n)`
///     by equivalence relationship `bi_pred`.
///   - Complexity: O(n). Maximum `n` bi_pred applications.
///
/// where n is number of elements in rng1.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3];
///
/// let is_eq = rng::equals_unbounded_by(&arr1, &arr2, arr2.start(), |x, y| y == x);
/// assert!(is_eq);
/// ```
pub fn equals_unbounded_by<R1, R2, BinaryPred>(
    rng1: &R1,
    rng2: &R2,
    start2: R2::Position,
    bi_pred: BinaryPred,
) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    BinaryPred: Fn(&R1::Element, &R2::Element) -> bool,
{
    algo::equals_unbounded_by(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        start2,
        bi_pred,
    )
}

/// Returns true if rng1 elements are equal to elements of rng2 starting from start2.
///
/// # Precondition
///   - `[start2, start2 + n)` denotes valid positions in rng2
///
/// # Postcondition
///   - Returns true if rng1 has equal elements as rng2 `[start2, start2 + n)`.
///   - Complexity: O(n). Maximum `n` equality comparisions.
///
/// where n is number of elements in rng1.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3];
///
/// let is_eq = rng::equals_unbounded(&arr1, &arr2, arr2.start());
/// assert!(is_eq);
/// ```
pub fn equals_unbounded<R1, R2>(
    rng1: &R1,
    rng2: &R2,
    start2: R2::Position,
) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    algo::equals_unbounded(rng1, rng1.start(), rng1.end(), rng2, start2)
}

/// Returns true if rng1 elements are equivalent to elements of rng2 by relationship bi_pred and have same length.
///
/// # Precondition
///   - bi_pred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns true if rng1 is equivalent to rng2 by relationship bi_pred and
///     both have same length.
///   - Complexity: O(n). Maximum `n` `bi_pred` applications.
///
/// Where n is min(#rng1, #rng2) and # is number of elements in range operator.
///
/// #### Infix Supported
/// YES
///
/// # Example
///
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3];
///
/// let is_eq = rng::equals_by(&arr1, &arr2, |x, y| x == y);
/// assert!(is_eq);
///
/// let is_eq = arr1.equals_by(&arr2, |x, y| x == y);
/// assert!(is_eq);
/// ```
pub fn equals_by<R1, R2, Binaryred>(
    rng1: &R1,
    rng2: &R2,
    bi_pred: Binaryred,
) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    Binaryred: Fn(&R1::Element, &R2::Element) -> bool,
{
    algo::equals_by(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
        bi_pred,
    )
}

/// Returns true if rng1 elements are equal to elements of rng2 and have same length.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if rng1 elements are equal to elements of rng2 and have same length.
///   - Complexity: O(n). Maximum `n` equality comparisions of elements.
///
/// Where n is min(#rng1, #rng2)) and # is number of elements in range operator.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3];
///
/// let is_eq = rng::equals(&arr1, &arr2);
/// assert!(is_eq);
///
/// let is_eq = arr1.equals(&arr2);
/// assert!(is_eq);
/// ```
pub fn equals<R1, R2>(rng1: &R1, rng2: &R2) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    algo::equals(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
    )
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    /// `equals`, `equals_by`.
    pub trait STLEqualsExt: InputRange {
        fn equals_by<R, F>(&self, rng: &R, bi_pred: F) -> bool
        where
            R: InputRange,
            F: Fn(&Self::Element, &R::Element) -> bool;

        fn equals<R>(&self, rng: &R) -> bool
        where
            R: InputRange,
            Self::Element: PartialEq<R::Element>;
    }

    impl<T> STLEqualsExt for T
    where
        T: InputRange + ?Sized,
    {
        fn equals_by<R, F>(&self, rng: &R, bi_pred: F) -> bool
        where
            R: InputRange,
            F: Fn(&Self::Element, &R::Element) -> bool,
        {
            rng::equals_by(self, rng, bi_pred)
        }

        fn equals<R>(&self, rng: &R) -> bool
        where
            R: InputRange,
            Self::Element: PartialEq<R::Element>,
        {
            rng::equals(self, rng)
        }
    }
}
