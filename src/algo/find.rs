// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, CollectionLifetime};

pub fn find_if<R, Pred>(rng: &R, pred: Pred) -> R::Position
where
    R: Collection + ?Sized,
    Pred: Fn(<R as CollectionLifetime<'_>>::Element) -> bool,
{
    let mut start = rng.start();
    let end = rng.end();
    while start != end {
        if rng.at(&start, &pred) {
            return start;
        }
        start = rng.after(start)
    }
    start
}

pub fn find_if_not<R, Pred>(rng: &R, pred: Pred) -> R::Position
where
    R: Collection + ?Sized,
    Pred: Fn(<R as CollectionLifetime<'_>>::Element) -> bool,
{
    find_if(rng, |x| !pred(x))
}
