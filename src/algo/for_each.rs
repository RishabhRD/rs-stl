// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, MutableCollection};

pub fn for_each<R, Op>(rng: &R, mut op: Op)
where
    R: Collection + ?Sized,
    for<'a> Op: FnMut(R::Element<'a>),
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
    for<'a> R::MutableSlice<'a>: MutableCollection,
    for<'a> Op: FnMut(R::MutableElement<'a>),
{
    let mut start = rng.start();
    let end = rng.end();
    while start != end {
        rng.at_mut(&start, &mut op);
        start = rng.after(start);
    }
}
