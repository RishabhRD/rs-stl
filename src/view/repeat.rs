// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

mod __details {
    use crate::{
        BidirectionalRange, ForwardRange, InputRange, RandomAccessRange, View,
    };

    #[derive(Clone)]
    pub struct RepeatView<T> {
        pub val: T,
    }

    impl<T> InputRange for RepeatView<T> {
        type Element = T;

        type Position = ();

        type ElementRef<'a>
            = &'a T
        where
            Self: 'a;

        fn start(&self) -> Self::Position {}

        fn is_end(&self, _: &Self::Position) -> bool {
            false
        }

        fn after(&self, _: Self::Position) -> Self::Position {}

        fn at<'a>(&'a self, _: &Self::Position) -> Self::ElementRef<'a> {
            &self.val
        }

        fn after_n(&self, _: Self::Position, _: usize) -> Self::Position {}
    }

    impl<T> View for RepeatView<T> {}

    impl<T> ForwardRange for RepeatView<T> {
        fn distance(&self, _: Self::Position, _: Self::Position) -> usize {
            0
        }
    }

    impl<T> BidirectionalRange for RepeatView<T> {
        fn before(&self, _: Self::Position) -> Self::Position {}

        fn before_n(&self, _: Self::Position, _: usize) -> Self::Position {}
    }

    impl<T> RandomAccessRange for RepeatView<T> {}
}

/// Returns a view that contains infinite stream of val.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view that contains infinite stream of val.
///   - The view satisfies following range traits:
///     - InputRange: YES
///     - BoundedRange: NO
///     - ForwardRange: YES
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
/// use view::infix::*;
///
/// let arr = view::repeat(2).take(3);
/// assert!(arr.equals(&[2, 2, 2]));
/// ```
pub fn repeat<T>(val: T) -> __details::RepeatView<T> {
    __details::RepeatView { val }
}
