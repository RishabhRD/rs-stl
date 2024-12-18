// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Counts elements from `[start, end)` in rng that satisfies predicate.
///
/// Precondition:
///   - `[start, end)` represent valid positions in rng.
///
/// Postcondition:
///   - Returns count of elements in `[start, end)` position of rng satisfying pred
///   - Complexity: O(n), Maximum `n` applications of `pred` where n is number of
///     elements in `[start, end)`.
pub fn count_if<Range, Pred>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Pred,
) -> u32
where
    Range: InputRange + ?Sized,
    Pred: Fn(&Range::Element) -> bool,
{
    let mut cnt: u32 = 0;
    while start != end {
        if pred(rng.at(&start)) {
            cnt += 1;
        }
        start = rng.after(start);
    }
    cnt
}

/// Counts elements from `[start, end)` in rng equals given element.
///
/// Precondition:
///   - `[start, end)` represent valid positions in rng.
///
/// Postcondition:
///   - Returns count of elements in `[start, end)` position of rng equals `e`
///   - Complexity: O(n), Maximum `n` applications of equality check
///     where n is number of elements in `[start, end)`.
pub fn count<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    e: &Range::Element,
) -> u32
where
    Range: InputRange + ?Sized,
    Range::Element: Eq,
{
    count_if(rng, start, end, |x| x == e)
}
