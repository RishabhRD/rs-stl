// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, OutputRange, Regular};

pub fn replace_if<R, F>(rng: &mut R, pred: F, new_e: &R::Element)
where
    R: OutputRange + ?Sized,
    R::Element: Clone,
    F: Fn(&R::Element) -> bool,
{
    algo::replace_if(rng, rng.start(), rng.end(), pred, new_e);
}

pub fn replace<R>(rng: &mut R, old_e: &R::Element, new_e: &R::Element)
where
    R: OutputRange + ?Sized,
    R::Element: Regular,
{
    algo::replace(rng, rng.start(), rng.end(), old_e, new_e)
}

pub mod infix {
    use crate::{rng, OutputRange, Regular};

    pub trait STLReplaceExt: OutputRange {
        fn replace_if<F>(&mut self, pred: F, new_e: &Self::Element)
        where
            F: Fn(&Self::Element) -> bool,
            Self::Element: Clone;

        fn replace(&mut self, old_e: &Self::Element, new_e: &Self::Element)
        where
            Self::Element: Regular;
    }

    impl<R> STLReplaceExt for R
    where
        R: OutputRange + ?Sized,
    {
        fn replace_if<F>(&mut self, pred: F, new_e: &Self::Element)
        where
            F: Fn(&Self::Element) -> bool,
            Self::Element: Clone,
        {
            rng::replace_if(self, pred, new_e)
        }

        fn replace(&mut self, old_e: &Self::Element, new_e: &Self::Element)
        where
            Self::Element: Regular,
        {
            rng::replace(self, old_e, new_e)
        }
    }
}
