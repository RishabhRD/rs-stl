// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, ForwardRange, OutputRange};

/// TODO: there are efficient implementations for BidirectionalRange and
/// RandomAccessRange in rust. How to overload for them in rust?
pub fn rotate<R>(rng: &mut R, mid: R::Position) -> R::Position
where
    R: OutputRange + ?Sized,
{
    algo::rotate(rng, rng.start(), mid, rng.end())
}

pub fn rotate_copy<R, D>(rng: &R, mid: R::Position, dest: &mut D) -> D::Position
where
    R: ForwardRange + ?Sized,
    R::Element: Clone,
    D: OutputRange<Element = R::Element> + ?Sized,
{
    algo::rotate_copy(rng, rng.start(), mid, rng.end(), dest, dest.start())
}

pub mod infix {
    use crate::{rng, OutputRange};

    /// TODO: there are efficient implementations for BidirectionalRange and
    /// RandomAccessRange in rust. How to overload for them in rust?
    pub trait STLRotateExt: OutputRange {
        fn rotate(&mut self, mid: Self::Position) -> Self::Position {
            rng::rotate(self, mid)
        }
    }

    impl<R> STLRotateExt for R where R: OutputRange + ?Sized {}
}
