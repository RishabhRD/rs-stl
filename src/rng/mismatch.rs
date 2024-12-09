// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

pub fn mismatch_by<R1, R2, F>(
    rng1: &R1,
    rng2: &R2,
    bi_pred: F,
) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    F: Fn(&R1::Element, &R2::Element) -> bool,
{
    algo::mismatch_by(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
        bi_pred,
    )
}

pub fn mismatch<R1, R2>(rng1: &R1, rng2: &R2) -> (R1::Position, R2::Position)
where
    R1: InputRange + ?Sized,
    R2: InputRange + ?Sized,
    R1::Element: PartialEq<R2::Element>,
{
    algo::mismatch(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
    )
}
