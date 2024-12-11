// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OutputRange, SemiRegular};

// Precondition:
//   - [start, end) represents valid position in rng.
//   - binary_pred should return true if 2 elements of rng are equivalent.
//   - binary_pred should follow equivalence relationship, otherwise behavior
//     is undefined.
// Postcondition:
//   - Removes all the adjacent equivalent elements from rng by moving them to
//     end of rng.
//   - NOTE: rng size would not be changed by this.
//   - Relative order of preserved elements are maintained.
//   - Relative order of removed elements are NOT guaranteed.
//   - Returns the position to new end of rng.
pub fn unique_by<R, F>(
    rng: &mut R,
    mut start: R::Position,
    end: R::Position,
    bi_pred: F,
) -> R::Position
where
    R: OutputRange + ?Sized,
    F: Fn(&R::Element, &R::Element) -> bool,
{
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

// Precondition:
//   - [start, end) represents valid position in rng.
//   - Equality comparision should follow equivalence relationship otherwise
//     behavior is undefined.
// Postcondition:
//   - Removes all the adjacent equivalent elements from rng by moving them to
//     end of rng.
//   - NOTE: rng size would not be changed by this.
//   - Relative order of preserved elements are maintained.
//   - Relative order of removed elements are NOT guaranteed.
//   - Returns the position to new end of rng.
pub fn unique<R>(
    rng: &mut R,
    start: R::Position,
    end: R::Position,
) -> R::Position
where
    R: OutputRange + ?Sized,
    R::Element: SemiRegular,
{
    unique_by(rng, start, end, |x, y| x == y)
}
