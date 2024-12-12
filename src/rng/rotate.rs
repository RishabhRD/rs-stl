// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, OutputRange};

// TODO: there are efficient implementations for BidirectionalRange and
// RandomAccessRange in rust. How to overload for them in rust?
pub fn rotate<R>(rng: &mut R, mid: R::Position) -> R::Position
where
    R: OutputRange + ?Sized,
{
    algo::rotate(rng, rng.start(), mid, rng.end())
}

pub mod infix {
    use crate::{rng, OutputRange};

    // TODO: there are efficient implementations for BidirectionalRange and
    // RandomAccessRange in rust. How to overload for them in rust?
    pub trait STLRotateExt: OutputRange {
        fn rotate(&mut self, mid: Self::Position) -> Self::Position {
            rng::rotate(self, mid)
        }
    }

    impl<R> STLRotateExt for R where R: OutputRange + ?Sized {}
}
