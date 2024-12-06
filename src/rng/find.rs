// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

pub fn find_if<T, F>(rng: &T, pred: F) -> T::Position
where
    T: InputRange,
    F: Fn(&T::Element) -> bool,
{
    algo::find_if(rng, rng.start(), rng.end(), pred)
}

pub mod infix {
    use crate::rng;
    use crate::InputRange;

    pub trait STLFindExt: InputRange {
        fn find_if<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool;
    }

    impl<T> STLFindExt for T
    where
        T: InputRange,
    {
        fn find_if<F>(&self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::find_if(self, pred)
        }
    }
}
