// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

pub fn equals_unbounded_by<R1, R2, F>(
    rng1: &R1,
    rng2: &R2,
    start2: R2::Position,
    bi_pred: F,
) -> bool
where
    R1: InputRange,
    R2: InputRange,
    F: Fn(&R1::Element, &R2::Element) -> bool,
{
    algo::equals_unbounded_by(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        start2,
        bi_pred,
    )
}

pub fn equals_unbounded<R1, R2>(
    rng1: &R1,
    rng2: &R2,
    start2: R2::Position,
) -> bool
where
    R1: InputRange,
    R2: InputRange,
    R1::Element: PartialEq<R2::Element>,
{
    algo::equals_unbounded(rng1, rng1.start(), rng1.end(), rng2, start2)
}

pub fn equals_by<R1, R2, F>(rng1: &R1, rng2: &R2, bi_pred: F) -> bool
where
    R1: InputRange,
    R2: InputRange,
    F: Fn(&R1::Element, &R2::Element) -> bool,
{
    algo::equals_by(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
        bi_pred,
    )
}

pub fn equals<R1, R2>(rng1: &R1, rng2: &R2) -> bool
where
    R1: InputRange,
    R2: InputRange,
    R1::Element: PartialEq<R2::Element>,
{
    algo::equals(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
    )
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    pub trait STLEqualsExt: InputRange {
        fn equals_by<R, F>(&self, rng: &R, bi_pred: F) -> bool
        where
            R: InputRange,
            F: Fn(&Self::Element, &R::Element) -> bool;

        fn equals<R>(&self, rng: &R) -> bool
        where
            R: InputRange,
            Self::Element: PartialEq<R::Element>;
    }

    impl<T> STLEqualsExt for T
    where
        T: InputRange,
    {
        fn equals_by<R, F>(&self, rng: &R, bi_pred: F) -> bool
        where
            R: InputRange,
            F: Fn(&Self::Element, &R::Element) -> bool,
        {
            rng::equals_by(self, rng, bi_pred)
        }

        fn equals<R>(&self, rng: &R) -> bool
        where
            R: InputRange,
            Self::Element: PartialEq<R::Element>,
        {
            rng::equals(self, rng)
        }
    }
}
