// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, OutputRange};

pub fn fill_value<R>(rng: &mut R, e: &R::Element)
where
    R: OutputRange + ?Sized,
    R::Element: Clone,
{
    algo::fill_value(rng, rng.start(), rng.end(), e);
}

pub fn fill_by<R, Gen>(rng: &mut R, gen: Gen)
where
    R: OutputRange + ?Sized,
    Gen: Fn() -> R::Element,
{
    algo::fill_by(rng, rng.start(), rng.end(), gen);
}

pub mod infix {
    use crate::{rng, OutputRange};

    pub trait STLFillExt: OutputRange {
        fn fill_value(&mut self, e: &Self::Element)
        where
            Self::Element: Clone;

        fn fill_by<Gen>(&mut self, gen: Gen)
        where
            Self::Element: Clone,
            Gen: Fn() -> Self::Element;
    }

    impl<R> STLFillExt for R
    where
        R: OutputRange + ?Sized,
    {
        fn fill_value(&mut self, e: &Self::Element)
        where
            Self::Element: Clone,
        {
            rng::fill_value(self, e)
        }

        fn fill_by<Gen>(&mut self, gen: Gen)
        where
            Self::Element: Clone,
            Gen: Fn() -> Self::Element,
        {
            rng::fill_by(self, gen)
        }
    }
}
