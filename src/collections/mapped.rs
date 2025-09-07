// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    value_ref::ValueRef, BidirectionalCollection, Collection, LazyCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

/// A lazy collection whose elements are applying closure on element-ref of `base`.
pub struct MappedCollection<Base, MapFn, MappedType>
where
    Base: Collection,
    MapFn: Fn(&Base::Element) -> MappedType,
{
    /// The base collection.
    pub base: Base,

    /// The mapping function.
    map_fn: MapFn,
}

impl<Base, MapFn, MappedType> MappedCollection<Base, MapFn, MappedType>
where
    Base: Collection,
    MapFn: Fn(&Base::Element) -> MappedType,
{
    pub(crate) fn new(base: Base, map_fn: MapFn) -> Self {
        MappedCollection { base, map_fn }
    }
}

impl<Base, MapFn, MappedType> Collection
    for MappedCollection<Base, MapFn, MappedType>
where
    Base: Collection,
    MapFn: Fn(&Base::Element) -> MappedType,
{
    type Position = Base::Position;

    type Element = MappedType;

    type ElementRef<'a>
        = ValueRef<MappedType>
    where
        Self: 'a;

    type Whole = Self;

    fn start(&self) -> Self::Position {
        self.base.start()
    }

    fn end(&self) -> Self::Position {
        self.base.end()
    }

    fn form_next(&self, position: &mut Self::Position) {
        self.base.form_next(position);
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        ValueRef::new((self.map_fn)(&self.base.at(i)))
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole> {
        Slice::new(self, from, to)
    }

    fn form_next_n(&self, position: &mut Self::Position, n: usize) {
        self.base.form_next_n(position, n);
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.base.form_next_n_limited_by(position, n, limit)
    }

    fn next(&self, position: Self::Position) -> Self::Position {
        self.base.next(position)
    }

    fn next_n(&self, position: Self::Position, n: usize) -> Self::Position {
        self.base.next_n(position, n)
    }

    fn next_n_limited_by(
        &self,
        position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        self.base.next_n_limited_by(position, n, limit)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.base.distance(from, to)
    }

    fn count(&self) -> usize {
        self.base.count()
    }

    fn underestimated_count(&self) -> usize {
        self.base.underestimated_count()
    }
}

impl<Base, MapFn, MappedType> LazyCollection
    for MappedCollection<Base, MapFn, MappedType>
where
    Base: Collection,
    MapFn: Fn(&Base::Element) -> MappedType,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        (self.map_fn)(&self.base.at(i))
    }
}

impl<Base, MapFn, MappedType> BidirectionalCollection
    for MappedCollection<Base, MapFn, MappedType>
where
    Base: BidirectionalCollection,
    Base::Whole: BidirectionalCollection,
    MapFn: Fn(&Base::Element) -> MappedType,
{
    fn form_prior(&self, position: &mut Self::Position) {
        self.base.form_prior(position);
    }

    fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
        self.base.form_next_n(position, n);
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.base.form_next_n_limited_by(position, n, limit)
    }

    fn prior(&self, position: Self::Position) -> Self::Position {
        self.base.prior(position)
    }

    fn prior_n(&self, position: Self::Position, n: usize) -> Self::Position {
        self.base.prior_n(position, n)
    }

    fn prior_n_limited_by(
        &self,
        position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        self.base.prior_n_limited_by(position, n, limit)
    }
}

impl<Base, MapFn, MappedType> RandomAccessCollection
    for MappedCollection<Base, MapFn, MappedType>
where
    Base: RandomAccessCollection,
    Base::Whole: RandomAccessCollection,
    MapFn: Fn(&Base::Element) -> MappedType,
{
}

impl<Base, MapFn, MappedType> ReorderableCollection
    for MappedCollection<Base, MapFn, MappedType>
where
    Base: ReorderableCollection,
    Base::Whole: ReorderableCollection,
    MapFn: Fn(&Base::Element) -> MappedType,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.base.swap_at(i, j)
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::Whole> {
        SliceMut::new(self, from, to)
    }
}

/// A lazy collection whose elements are applying closure on lazily computed values of `base`.
pub struct LazyMappedCollection<Base, MapFn, MappedType>
where
    Base: LazyCollection,
    Base::Whole: LazyCollection,
    MapFn: Fn(Base::Element) -> MappedType,
{
    /// The base collection.
    pub base: Base,

    /// The mapping function.
    map_fn: MapFn,
}

impl<Base, MapFn, MappedType> LazyMappedCollection<Base, MapFn, MappedType>
where
    Base: LazyCollection,
    Base::Whole: LazyCollection,
    MapFn: Fn(Base::Element) -> MappedType,
{
    pub(crate) fn new(base: Base, map_fn: MapFn) -> Self {
        LazyMappedCollection { base, map_fn }
    }
}

impl<Base, MapFn, MappedType> Collection
    for LazyMappedCollection<Base, MapFn, MappedType>
where
    Base: LazyCollection,
    Base::Whole: LazyCollection,
    MapFn: Fn(Base::Element) -> MappedType,
{
    type Position = Base::Position;

    type Element = MappedType;

    type ElementRef<'a>
        = ValueRef<MappedType>
    where
        Self: 'a;

    type Whole = Self;

    fn start(&self) -> Self::Position {
        self.base.start()
    }

    fn end(&self) -> Self::Position {
        self.base.end()
    }

    fn form_next(&self, position: &mut Self::Position) {
        self.base.form_next(position);
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        ValueRef::new((self.map_fn)(self.base.compute_at(i)))
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole> {
        Slice::new(self, from, to)
    }

    fn form_next_n(&self, position: &mut Self::Position, n: usize) {
        self.base.form_next_n(position, n);
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.base.form_next_n_limited_by(position, n, limit)
    }

    fn next(&self, position: Self::Position) -> Self::Position {
        self.base.next(position)
    }

    fn next_n(&self, position: Self::Position, n: usize) -> Self::Position {
        self.base.next_n(position, n)
    }

    fn next_n_limited_by(
        &self,
        position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        self.base.next_n_limited_by(position, n, limit)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.base.distance(from, to)
    }

    fn count(&self) -> usize {
        self.base.count()
    }

    fn underestimated_count(&self) -> usize {
        self.base.underestimated_count()
    }
}

impl<Base, MapFn, MappedType> LazyCollection
    for LazyMappedCollection<Base, MapFn, MappedType>
where
    Base: LazyCollection,
    Base::Whole: LazyCollection,
    MapFn: Fn(Base::Element) -> MappedType,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        (self.map_fn)(self.base.compute_at(i))
    }
}

impl<Base, MapFn, MappedType> BidirectionalCollection
    for LazyMappedCollection<Base, MapFn, MappedType>
where
    Base: BidirectionalCollection + LazyCollection,
    Base::Whole: BidirectionalCollection + LazyCollection,
    MapFn: Fn(Base::Element) -> MappedType,
{
    fn form_prior(&self, position: &mut Self::Position) {
        self.base.form_prior(position);
    }

    fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
        self.base.form_next_n(position, n);
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.base.form_next_n_limited_by(position, n, limit)
    }

    fn prior(&self, position: Self::Position) -> Self::Position {
        self.base.prior(position)
    }

    fn prior_n(&self, position: Self::Position, n: usize) -> Self::Position {
        self.base.prior_n(position, n)
    }

    fn prior_n_limited_by(
        &self,
        position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        self.base.prior_n_limited_by(position, n, limit)
    }
}

impl<Base, MapFn, MappedType> RandomAccessCollection
    for LazyMappedCollection<Base, MapFn, MappedType>
where
    Base: RandomAccessCollection + LazyCollection,
    Base::Whole: RandomAccessCollection + LazyCollection,
    MapFn: Fn(Base::Element) -> MappedType,
{
}

impl<Base, MapFn, MappedType> ReorderableCollection
    for LazyMappedCollection<Base, MapFn, MappedType>
where
    Base: ReorderableCollection + LazyCollection,
    Base::Whole: ReorderableCollection + LazyCollection,
    MapFn: Fn(Base::Element) -> MappedType,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.base.swap_at(i, j)
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::Whole> {
        SliceMut::new(self, from, to)
    }
}
