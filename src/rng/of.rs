// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Returns true if all elements in range satisfies the predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if all elements in rng satisfies pred
///   - Otherwise, returns false.
///   - Complexity: O(n), maximum n invocations of `pred`.
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
/// let arr = [1, 3, 5];
/// assert!(rng::all_of(&arr, |x| x % 2 == 1));
/// assert!(arr.all_of(|x| x % 2 == 1));
/// ```
pub fn all_of<Range, Pred>(rng: &Range, pred: Pred) -> bool
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if !pred(&rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }
    true
}

/// Returns true if atleast one element in range satisfies the predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if atleast one element in rng satisfies pred
///   - Otherwise, returns false.
///   - Complexity: O(n), maximum n invocations of `pred`.
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
/// let arr = [2, 4, 5];
/// assert!(rng::any_of(&arr, |x| x % 2 == 1));
/// assert!(arr.any_of(|x| x % 2 == 1));
/// ```
pub fn any_of<Range, Pred>(rng: &Range, pred: Pred) -> bool
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if pred(&rng.at(&start)) {
            return true;
        }
        start = rng.after(start);
    }
    false
}

/// Returns true if no element in range satisfies the predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if no element in rng satisfies pred.
///   - Otherwise, returns false.
///   - Complexity: O(n), maximum n invocations of `pred`.
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
/// let arr = [2, 4, 6];
/// assert!(rng::none_of(&arr, |x| x % 2 == 1));
/// assert!(arr.none_of(|x| x % 2 == 1));
/// ```
pub fn none_of<Range, Pred>(rng: &Range, pred: Pred) -> bool
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if pred(&rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }
    true
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    /// `all_of`, `any_of`, `none_of`.
    pub trait STLOfExt: InputRange {
        fn all_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool;

        fn any_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool;

        fn none_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool;
    }

    impl<T> STLOfExt for T
    where
        T: InputRange + ?Sized,
    {
        fn all_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::all_of(self, pred)
        }

        fn any_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::any_of(self, pred)
        }

        fn none_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::none_of(self, pred)
        }
    }
}
