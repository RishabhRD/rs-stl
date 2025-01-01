// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular};

/// Replaces elements of range satisfying predicate with new element.
///
/// # Precondition
///
/// # Poscondition:
///   - Replaces element of rng which satisfies pred with new_e.
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
/// let mut arr = [1, 2, 3];
/// rng::replace_if(&mut arr, |x| x % 2 == 1, &7);
/// assert!(arr.equals(&[7, 2, 7]));
///
/// let mut arr = [1, 2, 3];
/// arr.replace_if(|x| x % 2 == 1, &7);
/// assert!(arr.equals(&[7, 2, 7]));
/// ```
pub fn replace_if<Range, Pred>(
    rng: &mut Range,
    pred: Pred,
    new_e: &Range::Element,
) where
    Range: OutputRange + ?Sized,
    Range::Element: Clone,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if pred(&rng.at(&start)) {
            *rng.at_mut(&start) = new_e.clone();
        }
        start = rng.after(start);
    }
}

/// Replaces elements of range equals old element with new element.
///
/// # Precondition
///
/// # Poscondition:
///   - Replaces all elements == old_e with new_e of rng.
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
/// let mut arr = [1, 2, 1];
/// rng::replace(&mut arr, &1, &7);
/// assert!(arr.equals(&[7, 2, 7]));
///
/// let mut arr = [1, 2, 1];
/// arr.replace(&1, &7);
/// assert!(arr.equals(&[7, 2, 7]));
/// ```
pub fn replace<Range>(
    rng: &mut Range,
    old_e: &Range::Element,
    new_e: &Range::Element,
) where
    Range: OutputRange + ?Sized,
    Range::Element: Regular,
{
    replace_if(rng, |x| x == old_e, new_e)
}

/// Copies elements from src to dest with replacing the elements satisfying predicate with new
/// element.
///
/// # Precondition
///   - dest should be able to accomodate n elements.
///
/// # Poscondition:
///   - Copies elements from src to dest with replacing all elements
///     satisfying pred with new_e.
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
/// let arr = [1, 2, 1];
/// let mut dest = [0, 0, 0];
/// rng::replace_copy_if(&arr, &mut dest, |x| *x == 1, &7);
/// assert!(dest.equals(&[7, 2, 7]));
/// ```
pub fn replace_copy_if<SrcRange, DestRange, Pred>(
    src: &SrcRange,
    dest: &mut DestRange,
    pred: Pred,
    new_e: &SrcRange::Element,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    Pred: Fn(&SrcRange::Element) -> bool,
{
    let mut start = src.start();
    let mut out = dest.start();
    while !src.is_end(&start) {
        if pred(&src.at(&start)) {
            *dest.at_mut(&out) = new_e.clone();
        } else {
            *dest.at_mut(&out) = src.at(&start).clone();
        }
        out = dest.after(out);
        start = src.after(start);
    }
    out
}

/// Copies elements from src to dest with replacing the elements equals given element with new
/// element.
///
/// # Precondition
///   - dest should be able to accomodate elements being copied.
///
/// # Poscondition:
///   - Copies elements from src to dest with replacing all elements == old_e with new_e.
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
/// let arr = [1, 2, 1];
/// let mut dest = [0, 0, 0];
/// rng::replace_copy(&arr, &mut dest, &1, &7);
/// assert!(dest.equals(&[7, 2, 7]));
/// ```
pub fn replace_copy<SrcRange, DestRange>(
    src: &SrcRange,
    dest: &mut DestRange,
    old_e: &SrcRange::Element,
    new_e: &SrcRange::Element,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Regular,
{
    replace_copy_if(src, dest, |x| x == old_e, new_e)
}

pub mod infix {
    use crate::{rng, OutputRange, Regular};

    /// `replace`, `replace_if`.
    pub trait STLReplaceExt: OutputRange {
        fn replace_if<F>(&mut self, pred: F, new_e: &Self::Element)
        where
            F: Fn(&Self::Element) -> bool,
            Self::Element: Clone;

        fn replace(&mut self, old_e: &Self::Element, new_e: &Self::Element)
        where
            Self::Element: Regular;
    }

    impl<R> STLReplaceExt for R
    where
        R: OutputRange + ?Sized,
    {
        fn replace_if<F>(&mut self, pred: F, new_e: &Self::Element)
        where
            F: Fn(&Self::Element) -> bool,
            Self::Element: Clone,
        {
            rng::replace_if(self, pred, new_e)
        }

        fn replace(&mut self, old_e: &Self::Element, new_e: &Self::Element)
        where
            Self::Element: Regular,
        {
            rng::replace(self, old_e, new_e)
        }
    }
}
