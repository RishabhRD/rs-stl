// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::RandomAccessCollection;

/// Algorithms for `RandomAccessCollection`.
pub trait RandomAccessCollectionExt: RandomAccessCollection
where
    Self::Whole: RandomAccessCollection,
{
}

impl<R> RandomAccessCollectionExt for R
where
    R: RandomAccessCollection + ?Sized,
    R::Whole: RandomAccessCollection,
{
}
