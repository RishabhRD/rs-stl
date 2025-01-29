// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, Collection, LazyCollection, RandomAccessRange, Range,
    RangeBase, SubRangeable,
};

/// Slice of given range.
pub struct Slice<'a, R: Range> {
    m_range: &'a R,
    m_start: R::Position,
    m_end: R::Position,
}

impl<'a, R: Range> Slice<'a, R> {
    pub fn new(range: &'a R, start: R::Position, end: R::Position) -> Self {
        Self {
            m_range: range,
            m_start: start,
            m_end: end,
        }
    }
}

impl<R> RangeBase for Slice<'_, R>
where
    R: Range,
{
    type Position = R::Position;

    type Element = R::Element;
}

impl<R> SubRangeable<'_> for Slice<'_, R>
where
    R: Range,
{
    type SubRange = Self;
}

impl<R> Range for Slice<'_, R>
where
    R: Range,
{
    fn start(&self) -> Self::Position {
        self.m_start.clone()
    }

    fn end(&self) -> Self::Position {
        self.m_end.clone()
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        assert!(i != self.m_end);
        self.m_range.after(i)
    }

    fn slice(&self, from: Self::Position, to: Self::Position) -> Self {
        Self::new(self.m_range, from, to)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.m_range.after_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.m_range.distance(from, to)
    }

    fn at_as_deref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        (self.m_range).at_as_deref(i)
    }
}

impl<R> Collection for Slice<'_, R>
where
    R: Collection,
    for<'a> <R as SubRangeable<'a>>::SubRange: Collection,
{
    fn at(&self, i: &Self::Position) -> &Self::Element {
        assert!(*i != self.m_end);
        self.m_range.at(i)
    }
}

impl<R> LazyCollection for Slice<'_, R>
where
    R: LazyCollection,
    for<'a> <R as SubRangeable<'a>>::SubRange: LazyCollection,
{
    fn at(&self, i: &Self::Position) -> Self::Element {
        self.m_range.at(i)
    }
}

impl<R> BidirectionalRange for Slice<'_, R>
where
    R: BidirectionalRange,
    for<'a> <R as SubRangeable<'a>>::SubRange: BidirectionalRange,
{
    fn before(&self, i: Self::Position) -> Self::Position {
        assert!(i != self.m_start);
        self.m_range.before(i)
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.m_range.before_n(i, n)
    }
}

impl<R> RandomAccessRange for Slice<'_, R>
where
    R: RandomAccessRange,
    for<'a> <R as SubRangeable<'a>>::SubRange: RandomAccessRange,
{
}
