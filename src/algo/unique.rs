// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular, SemiRegular};

/// Moves all except first of adjacent equivalent elements by given equivalence relationship to end
/// of range.
///
/// Precondition:
///   - `[start, end)` represents valid position in rng.
///   - binary_pred should return true if 2 elements of rng are equivalent.
///   - binary_pred should follow equivalence relationship, otherwise behavior
///     is undefined.
///
/// Postcondition:
///   - Removes all the adjacent equivalent elements from rng by moving them to
///     end of rng.
///   - NOTE: rng size would not be changed by this.
///   - Relative order of preserved elements are maintained.
///   - Relative order of removed elements are NOT guaranteed.
///   - Returns the position to new end of rng.
///   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn unique_by<Range, BinaryPred>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    bi_pred: BinaryPred,
) -> Range::Position
where
    Range: OutputRange + ?Sized,
    BinaryPred: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return end;
    }
    let mut result = start.clone();
    start = rng.after(start);
    while start != end {
        if !bi_pred(rng.at(&result), rng.at(&start)) {
            result = rng.after(result);
            if result != start {
                rng.swap_at(&result, &start);
            }
        }
        start = rng.after(start);
    }
    rng.after(result)
}

/// Moves all except first of adjacent equal elements to end of range.
///
/// Precondition:
///   - `[start, end)` represents valid position in rng.
///   - Equality comparision should follow equivalence relationship otherwise
///     behavior is undefined.
///
/// Postcondition:
///   - Removes all the adjacent equivalent elements from rng by moving them to
///     end of rng.
///   - NOTE: rng size would not be changed by this.
///   - Relative order of preserved elements are maintained.
///   - Relative order of removed elements are NOT guaranteed.
///   - Returns the position to new end of rng.
///   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
pub fn unique<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) -> Range::Position
where
    Range: OutputRange + ?Sized,
    Range::Element: SemiRegular,
{
    unique_by(rng, start, end, |x, y| x == y)
}

/// Copies all elements from src to dest with copying only first of adjacent equivalent elements by
/// given equivalence relationship.
///
/// Precondition:
///   - `[start, end)` represents valid position in rng.
///   - dest can accomodate copied elements at out.
///   - binary_pred should return true if 2 elements of rng are equivalent.
///   - binary_pred should follow equivalence relationship, otherwise behavior
///     is undefined.
///
/// Postcondition:
///   - Copies elements from rng from `[start, end)` with omitting adjacent
///     equivalent elements by bi_pred to dest starting from out.
///   - Returns the position of past to last copied element in dest.
///   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
///
///   Where n is number of elements in `[start, end)`.
pub fn unique_copy_by<SrcRange, DestRange, BinaryPred>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    bi_pred: BinaryPred,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    BinaryPred: Fn(&SrcRange::Element, &SrcRange::Element) -> bool,
{
    if start == end {
        return out;
    }
    *dest.at_mut(&out) = src.at(&start).clone();
    start = src.after(start);
    while start != end {
        if !bi_pred(dest.at(&out), src.at(&start)) {
            out = dest.after(out);
            *dest.at_mut(&out) = src.at(&start).clone();
        }
        start = src.after(start);
    }
    dest.after(out)
}

/// Copies all elements from src to dest with copying only first of adjacent equal elements.
///
/// Precondition:
///   - `[start, end)` represents valid position in rng.
///   - dest can accomodate copied elements at out.
///   - R::Element equality comparision should follow equivalence relationsip
///     otherwise behavior is undefined.
///
/// Postcondition:
///   - Copies elements from rng from `[start, end)` with omitting adjacent
///     equivalent elements to dest starting from out.
///   - Returns the position of past to last copied element in dest.
///   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
pub fn unique_copy<SrcRange, DestRange>(
    src: &SrcRange,
    start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    out: DestRange::Position,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Regular,
{
    unique_copy_by(src, start, end, dest, out, |x, y| x == y)
}
