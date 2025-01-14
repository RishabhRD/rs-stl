// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular};

/// Replaces elements of range satisfying predicate with new element.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Poscondition:
///   - Replaces element which satisfies pred of rng from
///     `[start, end)` with new_e.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// let start = arr.start();
/// let end = arr.end();
/// algo::replace_if(&mut arr, start, end, |x| x % 2 == 1, &7);
/// assert!(arr.equals(&[7, 2, 7]));
/// ```
pub fn replace_if<Range, Pred>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Pred,
    new_e: &Range::Element,
) where
    Range: OutputRange + ?Sized,
    Range::Element: Clone,
    Pred: Fn(&Range::Element) -> bool,
{
    while start != end {
        if pred(&rng.at(&start)) {
            *rng.at_mut(&start) = new_e.clone();
        }
        start = rng.after(start);
    }
}

/// Replaces elements of range equals old element with new element.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Poscondition:
///   - Replaces all elements == old_e with new_e of rng from
///     `[start, end)`.
///   - Complexity: O(n). Exactly n equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 1];
/// let start = arr.start();
/// let end = arr.end();
/// algo::replace(&mut arr, start, end, &1, &7);
/// assert!(arr.equals(&[7, 2, 7]));
/// ```
pub fn replace<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    old_e: &Range::Element,
    new_e: &Range::Element,
) where
    Range: OutputRange + ?Sized,
    Range::Element: Regular,
{
    replace_if(rng, start, end, |x| x == old_e, new_e)
}

/// Copies elements from src to dest with replacing the elements satisfying predicate with new
/// element.
///
/// # Precondition
///   - `[start, end)` represents valid positions in src.
///   - dest should be able to accomodate n elements starting from out.
///
/// # Poscondition:
///   - Copies elements from `[start, end)` from src to new range dest starting
///     from out while replacing all elements satisfying pred with new_e.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [1, 2, 1];
/// let mut dest = [0, 0, 0];
/// let out = dest.start();
/// algo::replace_copy_if(&arr, arr.start(), arr.end(), &mut dest, out, |x| *x == 1, &7);
/// assert!(dest.equals(&[7, 2, 7]));
/// ```
pub fn replace_copy_if<SrcRange, DestRange, Pred>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    pred: Pred,
    new_e: &SrcRange::Element,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    Pred: Fn(&SrcRange::Element) -> bool,
{
    while start != end {
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
///   - `[start, end)` represents valid positions in src.
///   - dest should be able to accomodate elements being copied starting from out.
///
/// # Poscondition:
///   - Copies elements from `[start, end)` from src to new range dest starting
///     from out while replacing all elements == old_e with new_e.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [1, 2, 1];
/// let mut dest = [0, 0, 0];
/// let out = dest.start();
/// algo::replace_copy(&arr, arr.start(), arr.end(), &mut dest, out, &1, &7);
/// assert!(dest.equals(&[7, 2, 7]));
/// ```
pub fn replace_copy<SrcRange, DestRange>(
    src: &SrcRange,
    start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    out: DestRange::Position,
    old_e: &SrcRange::Element,
    new_e: &SrcRange::Element,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Regular,
{
    replace_copy_if(src, start, end, dest, out, |x| x == old_e, new_e)
}
