// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, CollectionLifetime, MutableCollection};

pub fn for_each<R, Op>(rng: &R, mut op: Op)
where
    R: Collection + ?Sized,
    Op: FnMut(<R as CollectionLifetime<'_>>::Element),
{
    let mut start = rng.start();
    let end = rng.end();
    while start != end {
        rng.at(&start, &mut op);
        start = rng.after(start);
    }
}

pub fn for_each_mut<R, Op>(rng: &mut R, mut op: Op)
where
    R: MutableCollection + ?Sized,
    for<'a> <R as CollectionLifetime<'a>>::MutableSlice: MutableCollection,
    Op: FnMut(<R as CollectionLifetime<'_>>::MutableElement),
{
    let mut start = rng.start();
    let end = rng.end();
    while start != end {
        rng.at_mut(&start, &mut op);
        start = rng.after(start);
    }
}
