// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular};

/// Moves all element satisfying predicate to end of range.
///
/// Precondition:
///   - `[start, end)` represents valid positions in rng
///
/// Postcondition:
///   - Removes element satisfying pred by moving them to end of rng.
///   - NOTE: remove_if doesn't resize rng.
///   - Relative ordering of elements NOT satisfying pred is preserved.
///   - Relative ordering of removed elements is NOT guaranteed.
///   - Returns end position for preserved elements in rng.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove_if<Range, Pred>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Pred,
) -> Range::Position
where
    Range: OutputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }
    if start != end {
        let mut i = rng.after(start.clone());
        while i != end {
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
/// Precondition:
///   - `[start, end)` represents valid positions in rng
///
/// Postcondition:
///   - Removes element == val by moving them to end of rng.
///   - NOTE: remove doesn't resize rng.
///   - Relative ordering of preserved elements are maintained.
///   - Relative ordering of removed elements is NOT guaranteed.
///   - Returns end position for preserved elements in rng.
///   - Complexity: O(n). Exactly n equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    val: &Range::Element,
) -> Range::Position
where
    Range: OutputRange + ?Sized,
    Range::Element: Eq,
{
    remove_if(rng, start, end, |x| x == val)
}

/// Copies elements from src to dest omitting elements satisfying given predicate.
///
/// Precondition:
///   - `[start, end)` represents valid positions in rng
///   - dest can accomodate copied elements starting from out.
///
/// Postcondition:
///   - Copies elements from `[start, end)` of `rng` to starting from `out`
///     of `dest`, omitting the elements which satisfies pred.
///   - Returns position of past the last element copied in dest.
///   - Complexity: O(n). Exactly n applications of pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove_copy_if<SrcRange, DestRange, Pred>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    pred: Pred,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    Pred: Fn(&SrcRange::Element) -> bool,
{
    while start != end {
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
/// Precondition:
///   - `[start, end)` represents valid positions in rng
///   - dest can accomodate copied elements starting from out.
///
/// Postcondition:
///   - Copies elements from `[start, end)` of `rng` to starting from `out`
///     of `dest`, omitting the elements == val.
///   - Returns position of past the last element copied in dest.
///   - Complexity: O(n). Exactly n equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
pub fn remove_copy<SrcRange, DestRange>(
    src: &SrcRange,
    start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    out: DestRange::Position,
    val: &SrcRange::Element,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    SrcRange::Element: Regular,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
{
    remove_copy_if(src, start, end, dest, out, |x| x == val)
}
