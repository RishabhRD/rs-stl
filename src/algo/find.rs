// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::Range;

pub fn find_if<R, Pred>(rng: &R, pred: Pred) -> R::Position
where
    R: Range + ?Sized,
    Pred: Fn(&R::Element) -> bool,
{
    let mut start = rng.start();
    let end = rng.end();
    while start != end {
        if pred(&rng.at_ref(&start)) {
            return start;
        }
        start = rng.after(start)
    }
    start
}

pub fn find_if_not<R, Pred>(rng: &R, pred: Pred) -> R::Position
where
    R: Range + ?Sized,
    Pred: Fn(&R::Element) -> bool,
{
    find_if(rng, |x| !pred(x))
}

pub fn find<R>(rng: &R, e: &R::Element) -> R::Position
where
    R: Range + ?Sized,
    R::Element: Eq,
{
    find_if(rng, |x| x == e)
}
