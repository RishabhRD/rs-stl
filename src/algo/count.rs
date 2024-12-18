// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

/// Precondition:
///   - `[start, end)` represent valid positions in rng.
///
/// Postcondition:
///   - Returns count of elements in `[start, end)` position of rng satisfying pred
///   - Complexity: O(n), Maximum `n` applications of `pred` where n is number of
///     elements in `[start, end)`.
pub fn count_if<R, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
) -> u32
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
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

/// Precondition:
///   - `[start, end)` represent valid positions in rng.
///
/// Postcondition:
///   - Returns count of elements in `[start, end)` position of rng equals `e`
///   - Complexity: O(n), Maximum `n` applications of equality check
///     where n is number of elements in `[start, end)`.
pub fn count<R>(
    rng: &R,
    start: R::Position,
    end: R::Position,
    e: &R::Element,
) -> u32
where
    R: InputRange + ?Sized,
    R::Element: Eq,
{
    count_if(rng, start, end, |x| x == e)
}
