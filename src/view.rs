// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # Views module
//!
//! The `view` module provides collection of views over given range.
//!
//! A view is just a range that implements `View` marker trait.
//!
//! For obtaining view from a range use `.view()` or `.view_mut()` function.
//!
//! ```rust
//! let arr = [1, 2, 3];
//! let v = arr.view(); // An immutable view of arr.
//!
//! let mut arr = [1, 2, 3];
//! let mut v = arr.mut_view(); // A mutable view of arr.
//! ```

use crate::{InputRange, SemiOutputRange};

pub mod view_details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct RangeView<'a, Range: InputRange + ?Sized> {
        pub range: &'a Range,
    }

    pub struct RangeMutView<'a, Range: InputRange + ?Sized> {
        pub range: &'a mut Range,
    }

    impl<'a, R> View for RangeView<'a, R> where R: InputRange + ?Sized {}
    impl<'a, R> View for RangeMutView<'a, R> where R: InputRange + ?Sized {}

    impl<'a, R> InputRange for RangeView<'a, R>
    where
        R: InputRange + ?Sized,
    {
        type Element = R::Element;

        type Position = R::Position;

        fn start(&self) -> Self::Position {
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn at(&self, i: &Self::Position) -> &Self::Element {
            self.range.at(i)
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

    impl<'a, R> InputRange for RangeMutView<'a, R>
    where
        R: InputRange + ?Sized,
    {
        type Element = R::Element;

        type Position = R::Position;

        fn start(&self) -> Self::Position {
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn at(&self, i: &Self::Position) -> &Self::Element {
            self.range.at(i)
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

    impl<'a, R> BoundedRange for RangeView<'a, R>
    where
        R: BoundedRange + ?Sized,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<'a, R> BoundedRange for RangeMutView<'a, R>
    where
        R: BoundedRange + ?Sized,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<'a, R> ForwardRange for RangeView<'a, R>
    where
        R: ForwardRange + ?Sized,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<'a, R> ForwardRange for RangeMutView<'a, R>
    where
        R: ForwardRange + ?Sized,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<'a, R> BidirectionalRange for RangeView<'a, R>
    where
        R: BidirectionalRange + ?Sized,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<'a, R> BidirectionalRange for RangeMutView<'a, R>
    where
        R: BidirectionalRange + ?Sized,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<'a, R> RandomAccessRange for RangeView<'a, R> where
        R: RandomAccessRange + ?Sized
    {
    }
    impl<'a, R> RandomAccessRange for RangeMutView<'a, R> where
        R: RandomAccessRange + ?Sized
    {
    }

    impl<'a, R> SemiOutputRange for RangeMutView<'a, R>
    where
        R: SemiOutputRange + ?Sized,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j);
        }
    }

    impl<'a, R> OutputRange for RangeMutView<'a, R>
    where
        R: OutputRange + ?Sized,
    {
        fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
            self.range.at_mut(i)
        }
    }
}

/// Provides `view` method for ranges.
#[doc(hidden)]
pub trait ViewExt: InputRange {
    fn view(&self) -> view_details::RangeView<Self> {
        view_details::RangeView { range: self }
    }
}
impl<R> ViewExt for R where R: InputRange {}

/// Provides `view_mut` method for ranges.
#[doc(hidden)]
pub trait MutableViewExt: SemiOutputRange {
    fn view_mut(&mut self) -> view_details::RangeMutView<Self> {
        view_details::RangeMutView { range: self }
    }
}

impl<R> MutableViewExt for R where R: SemiOutputRange {}

#[doc(hidden)]
pub mod subrange;
#[doc(inline)]
pub use subrange::*;

#[doc(hidden)]
pub mod prefix;
#[doc(inline)]
pub use prefix::*;
