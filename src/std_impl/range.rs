// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::ops::Range;
use std::ops::RangeInclusive;

use crate::{
    value_ref::ValueRef, BidirectionalCollection, Collection, LazyCollection,
    RandomAccessCollection, Slice,
};

macro_rules! impl_collection_for_range_inclusive {
($($t:ty),*) => {
  $(impl Collection for RangeInclusive<$t> {
      type Position = $t;

      type Element = $t;

      type ElementRef<'a>
          = ValueRef<$t>
      where
          Self: 'a;

      type Iter<'a>
          = std::iter::Map<Self, fn($t) -> ValueRef<$t>>
      where
          Self: 'a;

      type Whole = Self;

      fn start(&self) -> Self::Position {
          *self.start()
      }

      fn end(&self) -> Self::Position {
          *self.end() + 1
      }

      fn form_next(&self, position: &mut Self::Position) {
          *position += 1
      }

      fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
          ValueRef::new(*i)
      }

      fn slice(
          &self,
          from: Self::Position,
          to: Self::Position,
      ) -> crate::Slice<Self::Whole> {
          Slice::new(self, from, to)
      }

      fn form_next_n(&self, position: &mut Self::Position, n: usize) {
          *position += n as $t
      }

      fn form_next_n_limited_by(
          &self,
          position: &mut Self::Position,
          n: usize,
          limit: Self::Position,
      ) -> bool {
          if *position + n as Self::Position <= limit {
              *position += n as Self::Position;
              true
          } else {
              *position = limit;
              false
          }
      }

      fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
          (to - from) as usize
      }

       fn iter_within(
           &self,
           from: Self::Position,
           to: Self::Position,
       ) -> Self::Iter<'_> {
           (from..=(to - 1)).iter()
       }

      fn iter(&self) -> Self::Iter<'_> {
          self.clone().map(ValueRef::new)
      }
  }

  impl LazyCollection for RangeInclusive<$t> {
      type LazyIter<'a>
          = Self
      where
          Self: 'a;

      fn compute_at(&self, i: &Self::Position) -> Self::Element {
          *i
      }

      fn lazy_iter_within(
          &self,
          from: Self::Position,
          to: Self::Position,
      ) -> Self::LazyIter<'_> {
          from..=(to + 1)
      }

      fn lazy_iter(&self) -> Self::LazyIter<'_> {
          self.clone()
      }
  }

  impl BidirectionalCollection for RangeInclusive<$t> {
      fn form_prior(&self, position: &mut Self::Position) {
          *position -= 1
      }

      fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
          *position -= n as $t
      }

      fn form_prior_n_limited_by(
          &self,
          position: &mut Self::Position,
          n: usize,
          limit: Self::Position,
      ) -> bool {
          if *position - n as Self::Position >= limit {
              *position -= n as Self::Position;
              true
          } else {
              *position = limit;
              false
          }
      }
  }

  impl RandomAccessCollection for RangeInclusive<$t> {})*
};
}

impl_collection_for_range_inclusive!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! impl_collection_for_range {
($($t:ty),*) => {
  $(impl Collection for Range<$t> {
      type Position = $t;

      type Element = $t;

      type ElementRef<'a>
          = ValueRef<$t>
      where
          Self: 'a;

      type Whole = Self;

      type Iter<'a>
          = std::iter::Map<Self, fn($t) -> ValueRef<$t>>
      where
          Self: 'a;

      fn start(&self) -> Self::Position {
          self.start
      }

      fn end(&self) -> Self::Position {
          self.end
      }

      fn form_next(&self, position: &mut Self::Position) {
          *position += 1
      }

      fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
          ValueRef::new(*i)
      }

      fn slice(
          &self,
          from: Self::Position,
          to: Self::Position,
      ) -> crate::Slice<Self::Whole> {
          Slice::new(self, from, to)
      }

      fn form_next_n(&self, position: &mut Self::Position, n: usize) {
          *position += n as $t
      }

      fn form_next_n_limited_by(
          &self,
          position: &mut Self::Position,
          n: usize,
          limit: Self::Position,
      ) -> bool {
          if *position + n as Self::Position <= limit {
              *position += n as Self::Position;
              true
          } else {
              *position = limit;
              false
          }
      }

      fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
          (to - from) as usize
      }

      fn iter_within(
          &self,
          from: Self::Position,
          to: Self::Position,
      ) -> Self::Iter<'_> {
          (from..to).iter()
      }

      fn iter(&self) -> Self::Iter<'_> {
          self.clone().map(ValueRef::new)
      }
  }

  impl LazyCollection for Range<$t> {
      type LazyIter<'a>
          = Self
      where
          Self: 'a;

      fn compute_at(&self, i: &Self::Position) -> Self::Element {
          *i
      }

      fn lazy_iter_within(
          &self,
          from: Self::Position,
          to: Self::Position,
      ) -> Self::LazyIter<'_> {
          from..to
      }

      fn lazy_iter(&self) -> Self::LazyIter<'_> {
          self.clone()
      }
  }

  impl BidirectionalCollection for Range<$t> {
      fn form_prior(&self, position: &mut Self::Position) {
          *position -= 1
      }

      fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
          *position -= n as $t
      }

      fn form_prior_n_limited_by(
          &self,
          position: &mut Self::Position,
          n: usize,
          limit: Self::Position,
      ) -> bool {
          if *position - n as Self::Position >= limit {
              *position -= n as Self::Position;
              true
          } else {
              *position = limit;
              false
          }
      }
  }

  impl RandomAccessCollection for Range<$t> {})*
};
}

impl_collection_for_range!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
