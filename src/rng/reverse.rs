// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, BidirectionalRange, OutputRange};

pub fn reverse<R>(rng: &mut R)
where
    R: OutputRange + BidirectionalRange + ?Sized,
{
    algo::reverse(rng, rng.start(), rng.end())
}

pub mod infix {
    use crate::{rng, BidirectionalRange, OutputRange};

    pub trait STLReverseExt: OutputRange + BidirectionalRange {
        fn reverse(&mut self);
    }

    impl<R> STLReverseExt for R
    where
        R: OutputRange + BidirectionalRange + ?Sized,
    {
        fn reverse(&mut self) {
            rng::reverse(self)
        }
    }
}
