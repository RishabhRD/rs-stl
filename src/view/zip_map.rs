// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use std::cmp::min;

    use crate::{
        util::ValueRef, BidirectionalRange, BoundedRange, ForwardRange,
        InputRange, RandomAccessRange, SemiOutputRange, View,
    };

    #[derive(Clone)]
    pub struct ZipMapView<Range1, Range2, CombineOp, OutputElement>
    where
        Range1: InputRange + View,
        Range2: InputRange + View,
        CombineOp: Fn(&Range1::Element, &Range2::Element) -> OutputElement,
    {
        pub range1: Range1,
        pub range2: Range2,
        pub op: CombineOp,
    }

    impl<R1, R2, Op, O> InputRange for ZipMapView<R1, R2, Op, O>
    where
        R1: InputRange + View,
        R2: InputRange + View,
        Op: Fn(&R1::Element, &R2::Element) -> O,
    {
        type Element = O;

        type Position = (R1::Position, R2::Position);

        type ElementRef<'a>
            = ValueRef<O>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            (self.range1.start(), self.range2.start())
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range1.is_end(&i.0) || self.range2.is_end(&i.1)
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            (self.range1.after(i.0), self.range2.after(i.1))
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            ValueRef {
                val: (self.op)(&self.range1.at(&i.0), &self.range2.at(&i.1)),
            }
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            (self.range1.after_n(i.0, n), self.range2.after_n(i.1, n))
        }
    }

    impl<R1, R2, Op, O> View for ZipMapView<R1, R2, Op, O>
    where
        R1: InputRange + View,
        R2: InputRange + View,
        Op: Fn(&R1::Element, &R2::Element) -> O,
    {
    }

    impl<R1, R2, Op, O> BoundedRange for ZipMapView<R1, R2, Op, O>
    where
        R1: BoundedRange + View,
        R2: BoundedRange + View,
        Op: Fn(&R1::Element, &R2::Element) -> O,
    {
        fn end(&self) -> Self::Position {
            (self.range1.end(), self.range2.end())
        }
    }

    impl<R1, R2, Op, O> ForwardRange for ZipMapView<R1, R2, Op, O>
    where
        R1: ForwardRange + View,
        R2: ForwardRange + View,
        Op: Fn(&R1::Element, &R2::Element) -> O,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            min(
                self.range1.distance(from.0, to.0),
                self.range2.distance(from.1, to.1),
            )
        }
    }

    impl<R1, R2, Op, O> BidirectionalRange for ZipMapView<R1, R2, Op, O>
    where
        R1: BidirectionalRange + View,
        R2: BidirectionalRange + View,
        Op: Fn(&R1::Element, &R2::Element) -> O,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            (self.range1.before(i.0), self.range2.before(i.1))
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            (self.range1.before_n(i.0, n), self.range2.before_n(i.1, n))
        }
    }

    impl<R1, R2, Op, O> RandomAccessRange for ZipMapView<R1, R2, Op, O>
    where
        R1: RandomAccessRange + View,
        R2: RandomAccessRange + View,
        Op: Fn(&R1::Element, &R2::Element) -> O,
    {
    }

    impl<R1, R2, Op, O> SemiOutputRange for ZipMapView<R1, R2, Op, O>
    where
        R1: SemiOutputRange + View,
        R2: SemiOutputRange + View,
        Op: Fn(&R1::Element, &R2::Element) -> O,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range1.swap_at(&i.0, &j.0);
            self.range2.swap_at(&i.1, &j.1);
        }
    }
}

/// Zips the given views using given operation.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view that is zip of `view1` and `view2` using operation op.
///   - Range Traits satisfied:
///     - InputRange: YES
///     - BoundedRange: If view1 and view2 are BoundedRange
///     - ForwardRange: If view1 and view2 are ForwardRange
///     - BidirectionalRange: If view1 and view2 are BidirectionalRange
///     - RandomAccessRange: If view1 and view2 are RandomAccessRange
///     - SemiOutputRange: If view1 and view2 are SemiOutputRange
///     - OutputRange: NO
///
/// # Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut elements = [1, 2, 3];
/// let mut weights = [2, 1, 3];
///
/// view::zip_map(elements.view_mut(), weights.view_mut(), |_, y| *y)
///     .sort_range();
/// assert_eq!(elements, [2, 1, 3]);
/// assert_eq!(weights, [1, 2, 3]);
/// ```
pub fn zip_map<RangeView1, RangeView2, CombineOp, OutputElement>(
    view1: RangeView1,
    view2: RangeView2,
    op: CombineOp,
) -> __details::ZipMapView<RangeView1, RangeView2, CombineOp, OutputElement>
where
    RangeView1: InputRange + View,
    RangeView2: InputRange + View,
    CombineOp: Fn(&RangeView1::Element, &RangeView2::Element) -> OutputElement,
{
    __details::ZipMapView {
        range1: view1,
        range2: view2,
        op,
    }
}
