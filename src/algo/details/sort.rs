// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{RandomAccessRange, SemiOutputRange};

pub fn insertion_sort<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    cmp: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let mut i = start.clone();
    while i != end {
        let mut j = i.clone();
        while j != start && cmp(rng.at(&j), rng.at(&rng.before(j.clone()))) {
            let prev = rng.before(j.clone());
            rng.swap_at(&prev, &j);
            j = prev;
        }
        i = rng.after(i);
    }
}
