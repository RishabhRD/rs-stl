// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange, Regular, SemiRegular};

/// Moves all except first of adjacent equivalent elements by given equivalence relationship to end
/// of range.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///   - binary_pred should follow equivalence relationship, otherwise behavior
///     is undefined.
///
/// # Postcondition
///   - Removes all the adjacent equivalent elements from rng by moving them to
///     end of rng.
///   - NOTE: rng size would not be changed by this.
///   - Relative order of preserved elements are maintained.
///   - Relative order of removed elements are NOT guaranteed.
///   - Returns the position to new end of rng.
///   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 2), (1, 3), (2, 3)];
/// let i = algo::unique_by(&mut arr, 0, 3, |x, y| x.0 == y.0);
/// assert_eq!(i, 2);
/// assert!(arr[..i].equals(&[(1, 2), (2, 3)]));
/// ```
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
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///
/// # Postcondition
///   - Removes all the adjacent equivalent elements from rng by moving them to
///     end of rng.
///   - NOTE: rng size would not be changed by this.
///   - Relative order of preserved elements are maintained.
///   - Relative order of removed elements are NOT guaranteed.
///   - Returns the position to new end of rng.
///   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 1, 2];
/// let i = algo::unique(&mut arr, 0, 3);
/// assert_eq!(i, 2);
/// assert!(arr[..i].equals(&[1, 2]));
/// ```
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
/// # Precondition
///   - `[start, end)` represents valid position in src.
///   - dest can accomodate copied elements starting from out.
///   - binary_pred should follow equivalence relationship, otherwise behavior
///     is undefined.
///
/// # Postcondition
///   - Copies elements from src from `[start, end)` with omitting adjacent
///     equivalent elements by bi_pred to dest starting from out.
///   - Returns the position of past to last copied element in dest.
///   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [(1, 2), (1, 3), (2, 3)];
/// let mut dest = [(0, 0), (0, 0)];
/// let i = algo::unique_copy_by(&src, 0, 3, &mut dest, 0, |x, y| x.0 == y.0);
/// assert_eq!(i, 2);
/// assert!(dest.equals(&[(1, 2), (2, 3)]));
/// ```
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
/// # Precondition
///   - `[start, end)` represents valid position in src.
///   - dest can accomodate copied elements at out.
///
/// # Postcondition
///   - Copies elements from src from `[start, end)` with omitting adjacent
///     equal elements to dest starting from out.
///   - Returns the position of past to last copied element in dest.
///   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
///
///   Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 1, 2];
/// let mut dest = [0, 0];
/// let i = algo::unique_copy(&src, 0, 3, &mut dest, 0);
/// assert_eq!(i, 2);
/// assert!(dest.equals(&[1, 2]));
/// ```
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
