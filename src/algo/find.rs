// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Finds position of first element satisfying predicate.
///
/// # Precondition
///   - `[start, end)` represent valid positions in rng.
///
/// # Postcondition
///   - Returns position of first element in `[start, end)` of rng satisfying pred.
///   - Returns end if no such element exists.
///   - Complexity: O(n). Maximum `n` applications of `pred`,
///     where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 2, 3];
/// let i = algo::find_if(&arr, arr.start(), arr.end(), |x| *x == 3);
/// assert_eq!(i, 2);
/// ```
pub fn find_if<Range, Pred>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Pred,
) -> Range::Position
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    while start != end {
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
///   - `[start, end)` represent valid positions in rng.
///
/// # Postcondition
///   - Returns position of first element in `[start, end)` of rng NOT satisfying pred.
///   - Returns end if no such element exists.
///   - Complexity: O(n). Maximum `n` applications of `pred`,
///     where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 2, 3];
/// let i = algo::find_if_not(&arr, arr.start(), arr.end(), |x| *x == 3);
/// assert_eq!(i, 0);
/// ```
pub fn find_if_not<Range, Pred>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    pred: Pred,
) -> Range::Position
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    find_if(rng, start, end, |x| !pred(x))
}

/// Finds position of first element equals given element.
///
/// # Precondition
///   - `[start, end)` represent valid positions in rng.
///
/// # Postcondition
///   - Returns position of first element in `[start, end)` of rng equals e.
///   - Returns end if no such element exists.
///   - Complexity: O(n). Maximum `n` equality comparisions,
///     where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 2, 3];
/// let i = algo::find(&arr, arr.start(), arr.end(), &3);
/// assert_eq!(i, 2);
/// ```
pub fn find<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    e: &Range::Element,
) -> Range::Position
where
    Range: InputRange + ?Sized,
    Range::Element: Eq,
{
    find_if(rng, start, end, |x| x == e)
}
