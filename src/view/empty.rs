// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        RandomAccessRange, View,
    };

    #[derive(Clone)]
    pub struct EmptyView<T> {
        phantom: std::marker::PhantomData<T>,
    }

    impl<T> Default for EmptyView<T> {
        fn default() -> Self {
            EmptyView {
                phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T> InputRange for EmptyView<T> {
        type Element = T;

        type Position = ();

        type ElementRef<'a>
            = &'a T
        where
            Self: 'a;

        fn start(&self) -> Self::Position {}

        fn is_end(&self, _: &Self::Position) -> bool {
            true
        }

        fn after(&self, _: Self::Position) -> Self::Position {
            panic!("call to after on empty view");
        }

        fn at<'a>(&'a self, _: &Self::Position) -> Self::ElementRef<'a> {
            panic!("call to at on empty view");
        }

        fn after_n(&self, _: Self::Position, _: usize) -> Self::Position {
            panic!("call to after_n on empty view");
        }
    }

    impl<T> View for EmptyView<T> {}

    impl<T> BoundedRange for EmptyView<T> {
        fn end(&self) -> Self::Position {}
    }

    impl<T> ForwardRange for EmptyView<T> {
        fn distance(&self, _: Self::Position, _: Self::Position) -> usize {
            0
        }
    }

    impl<T> BidirectionalRange for EmptyView<T> {
        fn before(&self, _: Self::Position) -> Self::Position {
            panic!("call to before on empty view");
        }

        fn before_n(&self, _: Self::Position, _: usize) -> Self::Position {
            panic!("call to before_n on empty view");
        }
    }

    impl<T> RandomAccessRange for EmptyView<T> {}
}

/// Returns an empty view.
///
/// # Precondition
///
/// # Postcondition
///   - Returns an empty view.
///   - The view satisfies following range traits:
///     - InputRange: YES
///     - BoundedRange: YES
///     - BidirectionalRange: YES
///     - RandomAccessRange: YES
///     - SemiOutputRange: NO
///     - OutputRange: NO
///
/// # Infix Supported
/// NO
///
/// # Examples
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = view::empty::<i32>();
/// assert!(arr.equals(&[]));
/// ```
pub fn empty<T>() -> __details::EmptyView<T> {
    __details::EmptyView::default()
}
