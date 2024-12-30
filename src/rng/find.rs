// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

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
/// use rng::infix::*;
///
/// let arr = [1, 2, 3];
///
/// let i = rng::find_if(&arr, |x| *x == 3);
/// assert_eq!(i, 2);
///
/// let i = arr.find_if(|x| *x == 3);
/// assert_eq!(i, 2);
/// ```
pub fn find_if<Range, Pred>(rng: &Range, pred: Pred) -> Range::Position
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if pred(rng.at(&start)) {
            break;
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
/// use rng::infix::*;
///
/// let arr = [1, 2, 3];
///
/// let i = rng::find_if_not(&arr, |x| *x == 3);
/// assert_eq!(i, 0);
///
/// let i = arr.find_if_not(|x| *x == 3);
/// assert_eq!(i, 0);
/// ```
pub fn find_if_not<Range, Pred>(rng: &Range, pred: Pred) -> Range::Position
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
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
/// use rng::infix::*;
///
/// let arr = [1, 2, 3];
///
/// let i = rng::find(&arr, &3);
/// assert_eq!(i, 2);
///
/// let i = arr.find(&3);
/// assert_eq!(i, 2);
/// ```
pub fn find<Range>(rng: &Range, e: &Range::Element) -> Range::Position
where
    Range: InputRange + ?Sized,
    Range::Element: Eq,
{
    find_if(rng, |x| x == e)
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    /// `find_if`, `find_if_not`, `find`.
    pub trait STLFindExt: InputRange {
        fn find_if<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool;

        fn find_if_not<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool;

        fn find(&self, e: &Self::Element) -> Self::Position
        where
            Self::Element: Eq;
    }

    impl<T> STLFindExt for T
    where
        T: InputRange + ?Sized,
    {
        fn find_if<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::find_if(self, pred)
        }

        fn find_if_not<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::find_if_not(self, pred)
        }

        fn find(&self, e: &Self::Element) -> Self::Position
        where
            Self::Element: Eq,
        {
            rng::find(self, e)
        }
    }
}
