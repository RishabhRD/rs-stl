// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular, SemiOutputRange};

/// Moves all element satisfying predicate to end of range.
///
/// # Precondition
///
/// # Postcondition
///   - Removes element satisfying pred by moving them to end of rng.
///   - NOTE: remove_if doesn't resize rng.
///   - Relative ordering of elements NOT satisfying pred is preserved.
///   - Relative ordering of removed elements is NOT guaranteed.
///   - Returns end position for preserved elements in rng.
///   - Complexity: O(n). Exactly n applications of pred.
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
/// let mut arr = [1, 2, 3, 4];
/// let i = rng::remove_if(&mut arr, |x| x % 2 == 1);
/// assert_eq!(i, 2);
/// assert!(&arr[0..i].equals(&[2, 4]));
///
/// let mut arr = [1, 2, 3, 4];
/// let i = arr.remove_if(|x| x % 2 == 1);
/// assert_eq!(i, 2);
/// assert!(&arr[0..i].equals(&[2, 4]));
/// ```
pub fn remove_if<Range, Pred>(rng: &mut Range, pred: Pred) -> Range::Position
where
    Range: SemiOutputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if pred(rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }
    if !rng.is_end(&start) {
        let mut i = rng.after(start.clone());
        while !rng.is_end(&i) {
            if !pred(rng.at(&i)) {
                rng.swap_at(&i, &start);
                start = rng.after(start);
            }
            i = rng.after(i);
        }
    }

    start
}

/// Moves all element equals given element to end of range.
///
/// # Precondition
///
/// # Postcondition
///   - Removes element == val by moving them to end of rng.
///   - NOTE: remove doesn't resize rng.
///   - Relative ordering of preserved elements are maintained.
///   - Relative ordering of removed elements is NOT guaranteed.
///   - Returns end position for preserved elements in rng.
///   - Complexity: O(n). Exactly n equality comparisions.
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
/// let mut arr = [1, 2, 1, 4];
/// let i = rng::remove(&mut arr, &1);
/// assert_eq!(i, 2);
/// assert!(&arr[0..i].equals(&[2, 4]));
///
/// let mut arr = [1, 2, 1, 4];
/// let i = arr.remove(&1);
/// assert_eq!(i, 2);
/// assert!(&arr[0..i].equals(&[2, 4]));
/// ```
pub fn remove<Range>(rng: &mut Range, val: &Range::Element) -> Range::Position
where
    Range: SemiOutputRange + ?Sized,
    Range::Element: Eq,
{
    remove_if(rng, |x| x == val)
}

/// Copies elements from src to dest omitting elements satisfying given predicate.
///
/// # Precondition
///   - dest can accomodate copied elements.
///
/// # Postcondition
///   - Copies elements from `src` to `dest`, omitting the elements which
///     satisfies pred.
///   - Returns position of past the last element copied in dest.
///   - Complexity: O(n). Exactly n applications of pred.
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
/// let src = [1, 2, 3, 4];
///
/// let mut dest = [0, 0];
/// let i = rng::remove_copy_if(&src, &mut dest, |x| x % 2 == 1);
/// assert_eq!(i, 2);
/// assert!(dest.equals(&[2, 4]));
/// ```
pub fn remove_copy_if<SrcRange, DestRange, Pred>(
    src: &SrcRange,
    dest: &mut DestRange,
    pred: Pred,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    Pred: Fn(&SrcRange::Element) -> bool,
{
    let mut start = src.start();
    let mut out = dest.start();
    while !src.is_end(&start) {
        if !pred(src.at(&start)) {
            *dest.at_mut(&out) = src.at(&start).clone();
            out = dest.after(out);
        }
        start = src.after(start);
    }
    out
}

/// Copies elements from src to dest omitting elements equals given element.
///
/// # Precondition
///   - dest can accomodate copied elements.
///
/// # Postcondition
///   - Copies elements from `src` to `dest`, omitting the elements == val.
///   - Returns position of past the last element copied in dest.
///   - Complexity: O(n). Exactly n equality comparisions.
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
/// let src = [1, 2, 1, 4];
///
/// let mut dest = [0, 0];
/// let i = rng::remove_copy(&src, &mut dest, &1);
/// assert_eq!(i, 2);
/// assert!(dest.equals(&[2, 4]));
/// ```
pub fn remove_copy<R, D>(rng: &R, dest: &mut D, val: &R::Element) -> D::Position
where
    R: InputRange + ?Sized,
    R::Element: Regular,
    D: OutputRange<Element = R::Element> + ?Sized,
{
    remove_copy_if(rng, dest, |x| x == val)
}

pub mod infix {
    use crate::{rng, SemiOutputRange};

    /// `remove`, `remove_if`.
    pub trait STLRemoveExt: SemiOutputRange {
        fn remove_if<F>(&mut self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool;

        fn remove(&mut self, val: &Self::Element) -> Self::Position
        where
            Self::Element: Eq;
    }

    impl<R> STLRemoveExt for R
    where
        R: SemiOutputRange + ?Sized,
    {
        fn remove_if<F>(&mut self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::remove_if(self, pred)
        }

        fn remove(&mut self, val: &Self::Element) -> Self::Position
        where
            Self::Element: Eq,
        {
            rng::remove(self, val)
        }
    }
}
