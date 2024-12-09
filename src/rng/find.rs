// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

pub fn find_if<R, F>(rng: &R, pred: F) -> R::Position
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::find_if(rng, rng.start(), rng.end(), pred)
}

pub fn find_if_not<R, F>(rng: &R, pred: F) -> R::Position
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::find_if_not(rng, rng.start(), rng.end(), pred)
}

pub fn find<R>(rng: &R, e: &R::Element) -> R::Position
where
    R: InputRange + ?Sized,
    R::Element: Eq,
{
    algo::find(rng, rng.start(), rng.end(), e)
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    pub trait STLFindExt: InputRange {
        fn find_if<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool;

        fn find_if_not<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool;

        fn find(&self, e: &Self::Element) -> Self::Position
        where
            Self::Element: Eq;
    }

    impl<T> STLFindExt for T
    where
        T: InputRange + ?Sized,
    {
        fn find_if<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::find_if(self, pred)
        }

        fn find_if_not<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::find_if_not(self, pred)
        }

        fn find(&self, e: &Self::Element) -> Self::Position
        where
            Self::Element: Eq,
        {
            rng::find(self, e)
        }
    }
}
