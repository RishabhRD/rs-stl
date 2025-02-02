// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::Range;

/// Finds position of first element satisfying predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns position of first element in rng satisfying pred.
///   - Returns end position if no such element exists.
///   - Complexity: O(n). Maximum `n` applications of `pred`.
///
///     where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use algo::infix::*;
///
/// let arr = [1, 2, 3];
///
/// let i = algo::find_if(&arr, |x| *x == 3);
/// assert_eq!(i, 2);
///
/// let i = arr.find_if(|x| *x == 3);
/// assert_eq!(i, 2);
/// ```
pub fn find_if<R, Pred>(rng: &R, pred: Pred) -> R::Position
where
    R: Range + ?Sized,
    Pred: Fn(&R::Element) -> bool,
{
    let mut start = rng.start();
    let end = rng.end();
    while start != end {
        if pred(&rng.at_ref(&start)) {
            return start;
        }
        start = rng.after(start)
    }
    start
}

/// Finds position of first element not satisfying predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns position of first element in rng not satisfying pred.
///   - Returns end position if no such element exists.
///   - Complexity: O(n). Maximum `n` applications of `pred`.
///
///     where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use algo::infix::*;
///
/// let arr = [1, 2, 3];
///
/// let i = algo::find_if_not(&arr, |x| *x == 3);
/// assert_eq!(i, 0);
///
/// let i = arr.find_if_not(|x| *x == 3);
/// assert_eq!(i, 0);
/// ```
pub fn find_if_not<R, Pred>(rng: &R, pred: Pred) -> R::Position
where
    R: Range + ?Sized,
    Pred: Fn(&R::Element) -> bool,
{
    find_if(rng, |x| !pred(x))
}

/// Finds position of first element equals given element.
///
/// # Precondition
///
/// # Postcondition
///   - Returns position of first element in rng equals e.
///   - Returns end if no such element exists.
///   - Complexity: O(n). Maximum `n` equality comparisions,
///
///     where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use algo::infix::*;
///
/// let arr = [1, 2, 3];
///
/// let i = algo::find(&arr, &3);
/// assert_eq!(i, 2);
///
/// let i = arr.find(&3);
/// assert_eq!(i, 2);
/// ```
pub fn find<R>(rng: &R, e: &R::Element) -> R::Position
where
    R: Range + ?Sized,
    R::Element: Eq,
{
    find_if(rng, |x| x == e)
}

pub mod infix {
    use crate::Range;

    /// `find_if`, `find_if_not`, `find`.
    pub trait STLFindExt: Range {
        fn find_if<Pred>(&self, pred: Pred) -> Self::Position
        where
            Pred: Fn(&Self::Element) -> bool,
        {
            super::find_if(self, pred)
        }

        fn find_if_not<Pred>(&self, pred: Pred) -> Self::Position
        where
            Pred: Fn(&Self::Element) -> bool,
        {
            super::find_if_not(self, pred)
        }

        fn find(&self, e: &Self::Element) -> Self::Position
        where
            Self::Element: Eq,
        {
            super::find(self, e)
        }
    }

    impl<R> STLFindExt for R where R: Range + ?Sized {}
}
