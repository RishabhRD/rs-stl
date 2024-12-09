// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::InputRange;

// Precondition:
//   - [start, end) represent valid positions in rng.
// Postcondition:
//   - Returns position of first element in [start, end) of rng satisfying pred.
//   - Returns end if no such element exists.
//   - Complexity: O(n). Maximum `n` applications of `pred`,
//     where n is number of elements in [start, end).
pub fn find_if<R, F>(
    rng: &R,
    mut start: R::Position,
    end: R::Position,
    pred: F,
) -> R::Position
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    while start != end {
        if pred(rng.at(&start)) {
            break;
        }
        start = rng.after(start)
    }
    start
}

// Precondition:
//   - [start, end) represent valid positions in rng.
// Postcondition:
//   - Returns position of first element in [start, end) of rng NOT satisfying pred.
//   - Returns end if no such element exists.
//   - Complexity: O(n). Maximum `n` applications of `pred`,
//     where n is number of elements in [start, end).
pub fn find_if_not<R, F>(
    rng: &R,
    start: R::Position,
    end: R::Position,
    pred: F,
) -> R::Position
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    find_if(rng, start, end, |x| !pred(x))
}

// Precondition:
//   - [start, end) represent valid positions in rng.
// Postcondition:
//   - Returns position of first element in [start, end) of rng equals e.
//   - Returns end if no such element exists.
//   - Complexity: O(n). Maximum `n` equality comparisions,
//     where n is number of elements in [start, end).
pub fn find<R>(
    rng: &R,
    start: R::Position,
    end: R::Position,
    e: &R::Element,
) -> R::Position
where
    R: InputRange + ?Sized,
    R::Element: Eq,
{
    find_if(rng, start, end, |x| x == e)
}
