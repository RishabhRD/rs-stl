// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange, View,
    };

    #[derive(Clone)]
    pub struct JoinView<Element, InRange, OutRange>
    where
        InRange: InputRange<Element = Element>,
        for<'a> OutRange: 'a
            + InputRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
        pub range: OutRange,
    }

    #[derive(PartialEq, Eq, Clone)]
    pub enum JoinPosition<OutPos, InPos> {
        End(OutPos),
        Mid(OutPos, InPos),
    }

    fn find_previous<Element, InRange, OutRange>(
        rng: &JoinView<Element, InRange, OutRange>,
        mut out_pos: OutRange::Position,
    ) -> JoinPosition<OutRange::Position, InRange::Position>
    where
        InRange: BidirectionalRange<Element = Element> + BoundedRange,
        for<'a> OutRange: 'a
            + BidirectionalRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
        while out_pos != rng.range.start() {
            out_pos = rng.range.before(out_pos);
            let in_range: &InRange = rng.range.at(&out_pos);
            let in_pos = in_range.end();
            if in_pos != in_range.start() {
                return JoinPosition::Mid(out_pos, in_range.before(in_pos));
            }
            out_pos = rng.range.before(out_pos);
        }
        unreachable!("before called on start")
    }

    impl<Element, InRange, OutRange> InputRange
        for JoinView<Element, InRange, OutRange>
    where
        InRange: InputRange<Element = Element>,
        for<'a> OutRange: 'a
            + InputRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
        type Element = Element;

        type Position = JoinPosition<OutRange::Position, InRange::Position>;

        type ElementRef<'a>
            = InRange::ElementRef<'a>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            let mut out_start = self.range.start();
            while !self.range.is_end(&out_start) {
                let inner_range: &InRange = self.range.at(&out_start);
                let in_start = inner_range.start();
                if !inner_range.is_end(&in_start) {
                    return JoinPosition::Mid(out_start, in_start);
                }
                out_start = self.range.after(out_start);
            }
            JoinPosition::End(out_start)
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            match i {
                JoinPosition::End(j) => self.range.is_end(j),
                JoinPosition::Mid(_, _) => false,
            }
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            match i {
                JoinPosition::End(_) => unreachable!("after called at end"),
                JoinPosition::Mid(mut out_pos, mut in_pos) => {
                    let in_range: &InRange = self.range.at(&out_pos);
                    in_pos = in_range.after(in_pos);
                    if in_range.is_end(&in_pos) {
                        out_pos = self.range.after(out_pos);
                        while !self.range.is_end(&out_pos) {
                            let inner_range: &InRange = self.range.at(&out_pos);
                            let in_start = inner_range.start();
                            if !inner_range.is_end(&in_start) {
                                return JoinPosition::Mid(out_pos, in_start);
                            }
                            out_pos = self.range.after(out_pos)
                        }
                        JoinPosition::End(out_pos)
                    } else {
                        JoinPosition::Mid(out_pos, in_pos)
                    }
                }
            }
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            match i {
                JoinPosition::End(_) => unreachable!("at called at end"),
                JoinPosition::Mid(out_pos, in_pos) => {
                    let inner_range: &InRange = self.range.at(out_pos);
                    inner_range.at(in_pos)
                }
            }
        }
    }

    impl<Element, InRange, OutRange> View for JoinView<Element, InRange, OutRange>
    where
        InRange: InputRange<Element = Element>,
        for<'a> OutRange: 'a
            + InputRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
    }

    impl<Element, InRange, OutRange> BoundedRange
        for JoinView<Element, InRange, OutRange>
    where
        InRange: InputRange<Element = Element>,
        for<'a> OutRange: 'a
            + BoundedRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
        fn end(&self) -> Self::Position {
            JoinPosition::End(self.range.end())
        }
    }

    impl<Element, InRange, OutRange> ForwardRange
        for JoinView<Element, InRange, OutRange>
    where
        InRange: ForwardRange<Element = Element>,
        for<'a> OutRange: 'a
            + ForwardRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
    }

    impl<Element, InRange, OutRange> BidirectionalRange
        for JoinView<Element, InRange, OutRange>
    where
        InRange: BidirectionalRange<Element = Element> + BoundedRange,
        for<'a> OutRange: 'a
            + BidirectionalRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            match i {
                JoinPosition::End(out_pos) => find_previous(self, out_pos),
                JoinPosition::Mid(out_pos, in_pos) => {
                    let inner_range: &InRange = self.range.at(&out_pos);
                    if in_pos == inner_range.start() {
                        find_previous(self, out_pos)
                    } else {
                        JoinPosition::Mid(out_pos, inner_range.before(in_pos))
                    }
                }
            }
        }
    }
}

/// Returns a view that flattens a layer of nesting of given view.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view that flattens a layer of nesting of given view.
///   - Range traits satisfied:
///     - InputRange: YES
///     - BoundedRange: If OutRange is BoundedRange
///     - ForwardRange: If OutRange, InRange is ForwardRange
///     - BidirectionalRange: If OutRange is BidirectionalRange and InRange is BidirectionalRange +
///       BoundedRange
///     - RandomAccessRange: NO
///     - SemiOutputRange: NO
///     - OutputRange: NO
///
/// # Infix Supported
/// YES
///
/// # TODO
/// Join view requires, the inner range element ref to strictly be reference to
/// inner range. However, expressing that hits borrow checker current limitation
/// and borrow checker assumes it to be 'static right now. Due to this, test
/// cases are also using static array and makes join unusable at all. Instead
/// find a way to express the same without hitting borrow checker limitation.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
/// use view::infix::*;
///
/// static ARR: [[i32; 3]; 3] = [[2, 1, 2], [2, 9, 4], [3, 1, 2]];
///
/// let v = view::join(ARR.view());
/// assert_eq!(v.count(&2), 4);
///
/// let v = ARR.view().join();
/// assert_eq!(v.count(&2), 4);
/// ```
pub fn join<Element, InRange, OutRange>(
    view: OutRange,
) -> __details::JoinView<Element, InRange, OutRange>
where
    InRange: InputRange<Element = Element>,
    for<'a> OutRange:
        'a + InputRange<Element = InRange, ElementRef<'a> = &'a InRange> + View,
{
    __details::JoinView { range: view }
}

pub mod infix {
    use super::__details;
    use crate::{InputRange, View};

    pub trait STLJoinExt<Element, InRange>:
        InputRange<Element = InRange> + View + Sized
    where
        InRange: InputRange<Element = Element>,
    {
        fn join(self) -> __details::JoinView<Element, InRange, Self>
        where
            for<'a> Self: 'a
                + InputRange<Element = InRange, ElementRef<'a> = &'a InRange>
                + View,
        {
            super::join(self)
        }
    }

    impl<Element, InRange, T> STLJoinExt<Element, InRange> for T
    where
        T: InputRange<Element = InRange> + View,
        InRange: InputRange<Element = Element>,
        for<'a> T: 'a
            + InputRange<Element = InRange, ElementRef<'a> = &'a InRange>
            + View,
    {
    }
}
