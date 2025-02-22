// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    Collection, MutableCollection, MutableRange, RandomAccessRange, Range,
    RangeLifetime,
};

pub trait MutableRangeExtension: MutableRange
where
    for<'a> <Self as RangeLifetime<'a>>::Slice: Range,
    for<'a> <Self as RangeLifetime<'a>>::MutableSlice: MutableRange,
{
    /// Partitions range with partition_fn which accepts second argument as reference to element at i.
    fn partition_on<F>(
        &mut self,
        _i: &Self::Position,
        _partition_fn: F,
    ) -> Self::Position
    where
        F: Fn(&Self::Element, &Self::Element) -> bool,
    {
        todo!()
    }
}

pub trait RandomAccessRangeExtension: RandomAccessRange
where
    for<'a> <Self as RangeLifetime<'a>>::Slice: RandomAccessRange,
    for<'a> <Self as RangeLifetime<'a>>::MutableSlice: RandomAccessRange,
{
    fn quick_sort<Comparator>(&mut self, is_less: Comparator)
    where
        Self: MutableRange,
        for<'a> <Self as RangeLifetime<'a>>::MutableSlice: MutableRange,
        Comparator: Fn(&Self::Element, &Self::Element) -> bool,
    {
        let start = self.start();
        let mut end = self.end();
        if start == end {
            return;
        }
        end = self.before(end);
        let p = self.partition_on(&end, is_less);
        self.slice_mut(start, p.clone()).quick_sort(is_less);
        self.slice_mut(p, end).quick_sort(is_less);
    }
}

pub trait MutableCollectionExtension: MutableCollection
where
    for<'a> <Self as RangeLifetime<'a>>::Slice: Collection,
    for<'a> <Self as RangeLifetime<'a>>::MutableSlice: MutableCollection,
{
    fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self::Element),
    {
        let mut start = self.start();
        let end = self.end();
        while start != end {
            f(self.at_mut(&start));
            start = self.after(start);
        }
    }
}

impl<R> MutableCollectionExtension for R
where
    R: MutableCollection + ?Sized,
    for<'a> <R as RangeLifetime<'a>>::Slice: Collection,
    for<'a> <R as RangeLifetime<'a>>::MutableSlice: MutableCollection,
{
}

impl<R> MutableRangeExtension for R
where
    R: MutableRange + ?Sized,
    for<'a> <R as RangeLifetime<'a>>::Slice: Range,
    for<'a> <R as RangeLifetime<'a>>::MutableSlice: MutableRange,
{
}

impl<R> RandomAccessRangeExtension for R
where
    R: RandomAccessRange + ?Sized,
    for<'a> <R as RangeLifetime<'a>>::Slice: RandomAccessRange,
    for<'a> <R as RangeLifetime<'a>>::MutableSlice: RandomAccessRange,
{
}
