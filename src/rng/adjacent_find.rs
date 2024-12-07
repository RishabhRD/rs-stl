// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, ForwardRange, Regular};

pub fn adjacent_find_if<R, F>(rng: &R, bi_pred: F) -> R::Position
where
    R: ForwardRange,
    R::Position: Regular,
    F: Fn(&R::Element, &R::Element) -> bool,
{
    algo::adjacent_find_if(rng, rng.start(), rng.end(), bi_pred)
}

pub mod infix {
    use crate::{rng, ForwardRange, Regular};

    pub trait STLAdjacentFindExt: ForwardRange
    where
        Self::Position: Regular,
    {
        fn adjacent_find_if<F>(&self, bi_pred: F) -> Self::Position
        where
            F: Fn(&Self::Element, &Self::Element) -> bool;
    }

    impl<R> STLAdjacentFindExt for R
    where
        R: ForwardRange,
        R::Position: Regular,
    {
        fn adjacent_find_if<F>(&self, bi_pred: F) -> Self::Position
        where
            F: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::adjacent_find_if(self, bi_pred)
        }
    }
}
