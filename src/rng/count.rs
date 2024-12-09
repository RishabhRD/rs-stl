// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

pub fn count_if<R, F>(rng: &R, f: F) -> u32
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::count_if(rng, rng.start(), rng.end(), f)
}

pub fn count<R>(rng: &R, e: &R::Element) -> u32
where
    R: InputRange + ?Sized,
    R::Element: Eq,
{
    algo::count(rng, rng.start(), rng.end(), e)
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    pub trait STLCountExt: InputRange {
        fn count_if<F>(&self, pred: F) -> u32
        where
            F: Fn(&Self::Element) -> bool;

        fn count(&self, e: &Self::Element) -> u32
        where
            Self::Element: Eq;
    }

    impl<R> STLCountExt for R
    where
        R: InputRange + ?Sized,
    {
        fn count_if<F>(&self, pred: F) -> u32
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::count_if(self, pred)
        }

        fn count(&self, e: &Self::Element) -> u32
        where
            Self::Element: Eq,
        {
            rng::count(self, e)
        }
    }
}
