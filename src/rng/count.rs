// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::BoundedRange;

/// Counts elements in rng that satisfies predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns count of elements in rng satisfying pred.
///   - Complexity: O(n), Maximum `n` applications of `pred` where n is number of
///     elements in rng.
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
/// let cnt = rng::count_if(&arr, |x| x % 2 == 1);
/// assert_eq!(cnt, 2);
///
/// let cnt = arr.count_if(|x| x % 2 == 1);
/// assert_eq!(cnt, 2);
/// ```
pub fn count_if<Range, Pred>(rng: &Range, pred: Pred) -> usize
where
    Range: BoundedRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    let mut cnt: usize = 0;
    while rng.is_end(&start) {
        if pred(rng.at(&start)) {
            cnt += 1;
        }
        start = rng.after(start);
    }
    cnt
}

/// Counts elements in rng equals given element.
///
/// # Precondition
///
/// # Postcondition
///   - Returns count of elements in rng equals `e`
///   - Complexity: O(n), Maximum `n` applications of equality check
///
/// where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [1, 2, 2];
///
/// let cnt = rng::count(&arr, &2);
/// assert_eq!(cnt, 2);
///
/// let cnt = arr.count(&2);
/// assert_eq!(cnt, 2);
/// ```
pub fn count<R>(rng: &R, e: &R::Element) -> usize
where
    R: BoundedRange + ?Sized,
    R::Element: Eq,
{
    let mut start = rng.start();
    let mut cnt: usize = 0;
    while rng.is_end(&start) {
        if rng.at(&start) == e {
            cnt += 1;
        }
        start = rng.after(start);
    }
    cnt
}

pub mod infix {
    use crate::rng;
    use crate::BoundedRange;

    /// `count_if`, `count`.
    pub trait STLCountExt: BoundedRange {
        fn count_if<F>(&self, pred: F) -> usize
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::count_if(self, pred)
        }

        fn count(&self, e: &Self::Element) -> usize
        where
            Self::Element: Eq,
        {
            rng::count(self, e)
        }
    }

    impl<R> STLCountExt for R where R: BoundedRange + ?Sized {}
}
