// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange, OutputRange, Regular};

pub fn remove_if<R, F>(rng: &mut R, pred: F) -> R::Position
where
    R: OutputRange + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::remove_if(rng, rng.start(), rng.end(), pred)
}

pub fn remove<R>(rng: &mut R, val: &R::Element) -> R::Position
where
    R: OutputRange + ?Sized,
    R::Element: Eq,
{
    algo::remove(rng, rng.start(), rng.end(), val)
}

pub fn remove_copy_if<R, D, F>(rng: &R, dest: &mut D, pred: F) -> D::Position
where
    R: InputRange + ?Sized,
    R::Element: Clone,
    D: OutputRange<Element = R::Element> + ?Sized,
    F: Fn(&R::Element) -> bool,
{
    algo::remove_copy_if(rng, rng.start(), rng.end(), dest, dest.start(), pred)
}

pub fn remove_copy<R, D>(rng: &R, dest: &mut D, val: &R::Element) -> D::Position
where
    R: InputRange + ?Sized,
    R::Element: Regular,
    D: OutputRange<Element = R::Element> + ?Sized,
{
    algo::remove_copy(rng, rng.start(), rng.end(), dest, dest.start(), val)
}

pub mod infix {
    use crate::{rng, OutputRange};

    pub trait STLRemoveExt: OutputRange {
        fn remove_if<F>(&mut self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool;

        fn remove(&mut self, val: &Self::Element) -> Self::Position
        where
            Self::Element: Eq;
    }

    impl<R> STLRemoveExt for R
    where
        R: OutputRange + ?Sized,
    {
        fn remove_if<F>(&mut self, pred: F) -> Self::Position
        where
            F: Fn(&Self::Element) -> bool,
        {
            rng::remove_if(self, pred)
        }

        fn remove(&mut self, val: &Self::Element) -> Self::Position
        where
            Self::Element: Eq,
        {
            rng::remove(self, val)
        }
    }
}
