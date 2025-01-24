// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, BidirectionalRange, BoundedRange, InputRange, OutputRange};

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// left to right order.
///
/// # Precondition
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng
///     in left to right order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(rng.start(), rng.end())`.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = ["Hello", ", ", "World!"];
///
/// let res = rng::fold_left(&arr, String::from(""), |mut x, y|{
///   x.push_str(y);
///   x
/// });
/// assert_eq!(res, "Hello, World!");
///
/// let res = arr.fold_left(String::from(""), |mut x, y|{
///   x.push_str(y);
///   x
/// });
/// assert_eq!(res, "Hello, World!");
/// ```
pub fn fold_left<Range, Result, BinaryOp>(
    rng: &Range,
    mut init: Result,
    op: BinaryOp,
) -> Result
where
    Range: InputRange + ?Sized,
    BinaryOp: Fn(Result, &Range::Element) -> Result,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        init = op(init, &rng.at(&start));
        start = rng.after(start);
    }
    init
}

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// right to left order.
///
/// # Precondition
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng at
///     in right to left order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(rng.start(), rng.end())`.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = ["Hello", ", ", "World!"];
///
/// let res = rng::fold_right(&arr, String::from(""), |x, mut y|{
///   y.push_str(x);
///   y
/// });
/// assert_eq!(res, "World!, Hello");
///
/// let res = arr.fold_right(String::from(""), |x, mut y|{
///   y.push_str(x);
///   y
/// });
/// assert_eq!(res, "World!, Hello");
/// ```
pub fn fold_right<Range, Result, BinaryOp>(
    rng: &Range,
    init: Result,
    op: BinaryOp,
) -> Result
where
    Range: BidirectionalRange + BoundedRange + ?Sized,
    BinaryOp: Fn(&Range::Element, Result) -> Result,
{
    algo::fold_right(rng, rng.start(), rng.end(), init, op)
}

/// Modifies the given range such that it becomes the prefix sum by given operation of itself.
///
/// # Precondition
///
/// # Postcondition
///   - Modifies rng such that the resulting elements denote the prefix sum
///     by op of rng elements.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in rng.
///
/// # Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// rng::inclusive_scan_inplace(&mut arr, |x, y| x + y);
/// assert!(arr.equals(&[1, 3, 6]));
///
/// let mut arr = [1, 2, 3];
/// arr.inclusive_scan_inplace(|x, y| x + y);
/// assert!(arr.equals(&[1, 3, 6]));
/// ```
pub fn inclusive_scan_inplace<Range, BinaryOp>(rng: &mut Range, op: BinaryOp)
where
    Range: OutputRange + ?Sized,
    BinaryOp: Fn(&Range::Element, &Range::Element) -> Range::Element,
{
    let mut start = rng.start();
    if rng.is_end(&start) {
        return;
    }
    let mut prev = start.clone();
    start = rng.after(start);
    while !rng.is_end(&start) {
        let res = op(&rng.at(&prev), &rng.at(&start));
        *rng.at_mut(&start) = res;
        prev = start.clone();
        start = rng.after(start);
    }
}

/// Puts prefix sum by given operation of src range to dest range.
///
/// # Precondition
///   - out is valid position in dest.
///   - dest can accomodate n elements.
///
/// # Postcondition
///   - Puts prefix sum by op of src to dest.
///   - Returns the position in out after last element inserted.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in src.
///
/// # Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
/// let i  = rng::inclusive_scan(&src, &mut dest, |x, y| x + y);
/// assert_eq!(i, 3);
/// assert!(dest.equals(&[1, 3, 6]));
/// ```
pub fn inclusive_scan<SrcRange, DestRange, BinaryOp>(
    src: &SrcRange,
    dest: &mut DestRange,
    op: BinaryOp,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    BinaryOp: Fn(&DestRange::Element, &SrcRange::Element) -> DestRange::Element,
{
    let mut start = src.start();
    let mut out = dest.start();
    if src.is_end(&start) {
        return out;
    }
    *dest.at_mut(&out) = src.at(&start).clone();
    let mut prev = out.clone();
    out = dest.after(out);
    start = src.after(start);
    while !src.is_end(&start) {
        let res = op(&dest.at(&prev), &src.at(&start));
        *dest.at_mut(&out) = res;
        start = src.after(start);
        prev = out.clone();
        out = dest.after(out);
    }
    out
}

/// Mutates range such that it becomes exclusive prefix sum by given operation of itself.
///
/// # Precondition
///
/// # Postcondition
///   - Mutates rng such that first element becomes init, followed by generalized
///     prefix sum by op. Last element is not considered in prefix sum.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in rng.
///
/// # Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [2, 3, 7];
/// rng::exclusive_scan_inplace(&mut arr, 0, |x, y| x + y);
/// assert!(arr.equals(&[0, 2, 5]));
///
/// let mut arr = [2, 3, 7];
/// arr.exclusive_scan_inplace(0, |x, y| x + y);
/// assert!(arr.equals(&[0, 2, 5]));
/// ```
pub fn exclusive_scan_inplace<Range, BinaryOp>(
    rng: &mut Range,
    mut init: Range::Element,
    op: BinaryOp,
) where
    Range: OutputRange + ?Sized,
    BinaryOp: Fn(&Range::Element, &Range::Element) -> Range::Element,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        let val = init;
        init = op(&val, &rng.at(&start));
        *rng.at_mut(&start) = val;
        start = rng.after(start);
    }
}

/// Puts exclusive prefix sum by given operation of src range to dest range, i.e., ith element is
/// not considered in ith prefix sum.
///
/// # Precondition
///   - out is valid position in dest.
///   - dest can accomodate n elements.
///
/// # Postcondition
///   - Puts exclusive prefix sum by op of src to dest.
///   - Returns the position in out after last element inserted.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in src.
///
/// # Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
/// let i  =
///   rng::exclusive_scan(&src, &mut dest, 0, |x, y| x + y);
/// assert_eq!(i, 3);
/// assert!(dest.equals(&[0, 1, 3]));
/// ```
pub fn exclusive_scan<SrcRange, DestRange, BinaryOp>(
    src: &SrcRange,
    dest: &mut DestRange,
    mut init: DestRange::Element,
    op: BinaryOp,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    BinaryOp: Fn(&DestRange::Element, &SrcRange::Element) -> DestRange::Element,
{
    let mut start = src.start();
    let mut out = dest.start();
    while !src.is_end(&start) {
        let val = init;
        init = op(&val, &src.at(&start));
        start = src.after(start);
        *dest.at_mut(&out) = val;
        out = dest.after(out);
    }
    out
}

/// Computes the generalized inner product of two ranges using a combination and accumulation operation.
///
/// # Precondition
///
/// # Postcondition
///   - Returns the generalized inner product of rng1 and rng2 using `combine_op` and `reduce_op`.
///   - Complexity: O(n) applications of `combine_op` and `reduce_op`.

/// # Example
/// ```rust
/// use stl::*;
/// use rng::numeric::*;
///
/// let range1 = [1, 2, 3];
/// let range2 = [4, 5, 6];
/// let result = rng::inner_product(&range1, &range2, 0, |x, y| x * y, |acc, val| acc + val);
/// assert_eq!(result, 32); // (1*4 + 2*5 + 3*6 = 32)
/// ```
pub fn inner_product<Rng1, Rng2, T, U, CombineOp, ReduceOp>(
    rng1: &Rng1,
    rng2: &Rng2,
    mut init: T,
    combine_op: CombineOp,
    reduce_op: ReduceOp,
) -> T
where
    Rng1: InputRange + ?Sized,
    Rng2: InputRange + ?Sized,
    CombineOp: Fn(&Rng1::Element, &Rng2::Element) -> U,
    ReduceOp: Fn(T, U) -> T,
{
    let mut start1 = rng1.start();
    let mut start2 = rng2.start();

    while !rng1.is_end(&start1) && !rng2.is_end(&start2) {
        let elem1 = rng1.at(&start1);
        let elem2 = rng2.at(&start2);

        let combined = combine_op(&elem1, &elem2);
        init = reduce_op(init, combined);

        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }

    init
}

pub mod infix {
    use crate::{
        rng, BidirectionalRange, BoundedRange, InputRange, OutputRange,
    };

    /// `fold_left`.
    pub trait STLNumericInputExt: InputRange {
        fn fold_left<Result, BinaryOp>(
            &self,
            init: Result,
            op: BinaryOp,
        ) -> Result
        where
            BinaryOp: Fn(Result, &Self::Element) -> Result,
        {
            rng::fold_left(self, init, op)
        }
    }

    impl<R> STLNumericInputExt for R where R: InputRange + ?Sized {}

    /// `fold_right`.
    pub trait STLNumericBidirExt: BidirectionalRange + BoundedRange {
        fn fold_right<Result, BinaryOp>(
            &self,
            init: Result,
            op: BinaryOp,
        ) -> Result
        where
            BinaryOp: Fn(&Self::Element, Result) -> Result,
        {
            rng::fold_right(self, init, op)
        }
    }

    impl<R> STLNumericBidirExt for R where
        R: BidirectionalRange + BoundedRange + ?Sized
    {
    }

    /// `inclusive_scan_inplace`, `exclusive_scan_inplace`.
    pub trait STLNumericOutputExt: OutputRange {
        fn inclusive_scan_inplace<BinaryOp>(&mut self, op: BinaryOp)
        where
            BinaryOp: Fn(&Self::Element, &Self::Element) -> Self::Element,
        {
            super::inclusive_scan_inplace(self, op);
        }

        fn exclusive_scan_inplace<BinaryOp>(
            &mut self,
            init: Self::Element,
            op: BinaryOp,
        ) where
            BinaryOp: Fn(&Self::Element, &Self::Element) -> Self::Element,
        {
            super::exclusive_scan_inplace(self, init, op);
        }
    }

    impl<R> STLNumericOutputExt for R where R: OutputRange + ?Sized {}
}
