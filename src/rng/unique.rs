// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, OutputRange, SemiRegular};

pub fn unique_by<R, F>(rng: &mut R, bi_pred: F) -> R::Position
where
    R: OutputRange + ?Sized,
    F: Fn(&R::Element, &R::Element) -> bool,
{
    algo::unique_by(rng, rng.start(), rng.end(), bi_pred)
}

pub fn unique<R>(rng: &mut R) -> R::Position
where
    R: OutputRange + ?Sized,
    R::Element: SemiRegular,
{
    algo::unique(rng, rng.start(), rng.end())
}

pub mod infix {
    use crate::{rng, OutputRange, SemiRegular};

    pub trait STLUniqueExt: OutputRange {
        fn unique_by<F>(&mut self, bi_pred: F) -> Self::Position
        where
            F: Fn(&Self::Element, &Self::Element) -> bool;

        fn unique(&mut self) -> Self::Position
        where
            Self::Element: SemiRegular;
    }

    impl<R> STLUniqueExt for R
    where
        R: OutputRange + ?Sized,
    {
        fn unique_by<F>(&mut self, bi_pred: F) -> Self::Position
        where
            F: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::unique_by(self, bi_pred)
        }

        fn unique(&mut self) -> Self::Position
        where
            Self::Element: SemiRegular,
        {
            rng::unique(self)
        }
    }
}
