// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange, OutputRange};

pub fn copy_if<R, D, F>(rng: &R, dest: &mut D, pred: F) -> D::Position
where
    R: InputRange<Element = D::Element>,
    R::Element: Clone,
    D: OutputRange,
    F: Fn(&R::Element) -> bool,
{
    algo::copy_if(rng, rng.start(), rng.end(), dest, dest.start(), pred)
}

pub fn copy<R, D>(rng: &R, dest: &mut D) -> D::Position
where
    R: InputRange<Element = D::Element>,
    R::Element: Clone,
    D: OutputRange,
{
    algo::copy(rng, rng.start(), rng.end(), dest, dest.start())
}
