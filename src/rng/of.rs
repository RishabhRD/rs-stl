// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

pub fn all_of<R, F>(rng: &R, pred: F) -> bool
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::all_of(rng, rng.start(), rng.end(), pred)
}

pub fn any_of<R, F>(rng: &R, pred: F) -> bool
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::any_of(rng, rng.start(), rng.end(), pred)
}

pub fn none_of<R, F>(rng: &R, pred: F) -> bool
where
    R: InputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::none_of(rng, rng.start(), rng.end(), pred)
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    pub trait STLOfExt: InputRange {
        fn all_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool;

        fn any_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool;

        fn none_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool;
    }

    impl<T> STLOfExt for T
    where
        T: InputRange + ?Sized,
    {
        fn all_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::all_of(self, pred)
        }

        fn any_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::any_of(self, pred)
        }

        fn none_of<F>(&self, pred: F) -> bool
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::none_of(self, pred)
        }
    }
}
