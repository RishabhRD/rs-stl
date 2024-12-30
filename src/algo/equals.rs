// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Returns true if range1 elements are equivalent to prefix of range2 elements wrt given
/// equivalence relationship.
///
/// # Precondition
///   - `[start1, end1)` denotes valid positions in rng1.
///   - `[start2, start2 + n)` denotes valid positions in rng2.
///   - BinaryPred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns true if rng1 `[start1, end1) is equivalent to prefix of
///     rng2 `[start2, end2) wrt bi_pred.
///   - Complexity: O(n). Maximum `n` bi_pred applications.
///
/// where n is `min(#[start1, end1), #[start2, end2))`
/// and # is number of elements operator.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3, 4];
///
/// let is_eq = algo::equals_prefix_by(&arr1, arr1.start(), arr1.end(), &arr2, arr2.start(), arr2.end(), |x, y| y == x);
/// assert!(is_eq);
/// ```
pub fn equals_prefix_by<R1, R2, BinaryPred>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    end2: R2::Position,
    bi_pred: BinaryPred,
) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    BinaryPred: Fn(&R1::Element, &R2::Element) -> bool,
{
    while start1 != end1 && start2 != end2 {
        if !bi_pred(rng1.at(&start1), rng2.at(&start2)) {
            return false;
        }
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }
    start1 == end1
}

/// Returns true if range1 elements are equal to prefix of range2 elements.
///
/// # Precondition
///   - `[start1, end1)` denotes valid positions in rng1.
///   - `[start2, start2 + n)` denotes valid positions in rng2.
///
/// # Postcondition
///   - Returns true if rng1 at `[start1, end1)` is equal to
///     prefix of rng2 at `[start2, end2)`.
///   - Complexity: O(n). Maximum `n` equality comparisions.
///
/// where n is `min(#[start1, end1), #[start2, end2))`
/// and # is number of elements operator.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3, 4];
///
/// let is_eq = algo::equals_prefix(&arr1, arr1.start(), arr1.end(), &arr2, arr2.start(),
/// arr2.end());
/// assert!(is_eq);
/// ```
pub fn equals_prefix<R1, R2>(
    rng1: &R1,
    start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    start2: R2::Position,
    end2: R2::Position,
) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    equals_prefix_by(rng1, start1, end1, rng2, start2, end2, |x, y| x == y)
}

/// Returns true if rng1 `[start1, end1)` elements are equivalent to elements of rng2 `[start2, end2)`
/// by relationship bi_pred and have same length.
///
/// # Precondition
///   - `[start1, end1)` denotes valid positions in rng1.
///   - `[start2, end2)` denotes valid positions in rng2
///   - bi_pred should follow equivalence relationship.
///
/// # Postcondition
///   - Returns true if range at `[start1, end1)` is equivalent to
///     range at `[start2, end2)` by relationship bi_pred.
///   - Complexity: O(n). Maximum `n` `bi_pred` applications.
///     Where n is min(#`[start1, end1)`, #`[start2, end2)`) and
///     # is number of elements in range operator.
///
/// # Example
///
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3];
///
/// let is_eq = algo::equals_by(&arr1, arr1.start(), arr1.end(), &arr2, arr2.start(), arr2.end(), |x, y| x == y);
/// assert!(is_eq);
/// ```
pub fn equals_by<R1, R2, F>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    end2: R2::Position,
    bi_pred: F,
) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    F: Fn(&R1::Element, &R2::Element) -> bool,
{
    while start1 != end1 && start2 != end2 {
        if !bi_pred(rng1.at(&start1), rng2.at(&start2)) {
            return false;
        }
        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }
    start1 == end1 && start2 == end2
}

/// Returns true if rng1 `[start1, end1)` elements are equal to elements of rng2 `[start2, end2)`
/// and have same length.
///
/// # Precondition
///   - `[start1, end1)` denotes valid positions in rng1.
///   - `[start2, end2)` denotes valid positions in rng2
///
/// # Postcondition
///   - Returns true if range at `[start1, end1)` is equal to
///     range at [start2, end2).
///   - Complexity: O(n). Maximum `n` equality comparisions of elements.
///
/// Where n is min(#`[start1, end1)`, #`[start2, end2)`) and
/// # is number of elements in range operator.
///
/// # Example
///
/// ```rust
/// use stl::*;
///
/// let arr1 = [1, 2, 3];
/// let arr2 = [1, 2, 3];
///
/// let is_eq = algo::equals(&arr1, arr1.start(), arr1.end(), &arr2, arr2.start(), arr2.end());
/// assert!(is_eq);
/// ```
pub fn equals<R1, R2>(
    rng1: &R1,
    start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    start2: R2::Position,
    end2: R2::Position,
) -> bool
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    equals_by(rng1, start1, end1, rng2, start2, end2, |x, y| x == y)
}
