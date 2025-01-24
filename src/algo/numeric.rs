// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, InputRange, OutputRange};

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// left to right order.
///
/// # Precondition
///   - `[start, end)` denotes valid positions in rng.
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng at
///     `[start, end)` in left to right order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = ["Hello", ", ", "World!"];
/// let res = algo::fold_left(&arr, arr.start(), arr.end(), String::from(""), |mut x, y|{
///   x.push_str(y);
///   x
/// });
/// assert_eq!(res, "Hello, World!");
/// ```
pub fn fold_left<Range, Result, BinaryOp>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    mut init: Result,
    op: BinaryOp,
) -> Result
where
    Range: InputRange + ?Sized,
    BinaryOp: Fn(Result, &Range::Element) -> Result,
{
    while start != end {
        init = op(init, &rng.at(&start));
        start = rng.after(start);
    }
    init
}

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// right to left order.
///
/// # Precondition
///   - `[start, end)` denotes valid positions in rng.
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng at
///     `[start, end)` in right to left order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = ["Hello", ", ", "World!"];
/// let res = algo::fold_right(&arr, arr.start(), arr.end(), String::from(""), |x, mut y|{
///   y.push_str(x);
///   y
/// });
/// assert_eq!(res, "World!, Hello");
/// ```
pub fn fold_right<Range, Result, BinaryOp>(
    rng: &Range,
    start: Range::Position,
    mut end: Range::Position,
    mut init: Result,
    op: BinaryOp,
) -> Result
where
    Range: BidirectionalRange + ?Sized,
    BinaryOp: Fn(&Range::Element, Result) -> Result,
{
    while start != end {
        end = rng.before(end);
        init = op(&rng.at(&end), init);
    }
    init
}

/// Modifies the given range such that it becomes the prefix sum by given operation of itself.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Modifies rng at `[start, end)` such that the resulting elements denote
///     the prefix sum by op of rng `[start, end)` elements.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// let start = arr.start();
/// let end = arr.end();
/// algo::inclusive_scan_inplace(&mut arr, start, end, |x, y| x + y);
/// assert!(arr.equals(&[1, 3, 6]));
/// ```
pub fn inclusive_scan_inplace<Range, BinaryOp>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    op: BinaryOp,
) where
    Range: OutputRange + ?Sized,
    BinaryOp: Fn(&Range::Element, &Range::Element) -> Range::Element,
{
    if start == end {
        return;
    }
    let mut prev = start.clone();
    start = rng.after(start);
    while start != end {
        let res = op(&rng.at(&prev), &rng.at(&start));
        *rng.at_mut(&start) = res;
        prev = start.clone();
        start = rng.after(start);
    }
}

/// Puts prefix sum by given operation of src range to dest range.
///
/// # Precondition
///   - `[start, end)` represents valid positions in src.
///   - out is valid position in dest.
///   - dest can accomodate n elements.
///
/// # Postcondition
///   - Puts prefix sum by op of src `[start, end)` to dest starting from out.
///   - Returns the position in out after last element inserted.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in src.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
/// let out = dest.start();
/// let i  =
///   algo::inclusive_scan(&src, src.start(), src.end(), &mut dest, out, |x, y| x + y);
/// assert_eq!(i, 3);
/// assert!(dest.equals(&[1, 3, 6]));
/// ```
pub fn inclusive_scan<SrcRange, DestRange, BinaryOp>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    op: BinaryOp,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    BinaryOp: Fn(&DestRange::Element, &SrcRange::Element) -> DestRange::Element,
{
    if start == end {
        return out;
    }
    *dest.at_mut(&out) = src.at(&start).clone();
    let mut prev = out.clone();
    out = dest.after(out);
    start = src.after(start);
    while start != end {
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
///   - `[start, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Mutates rng such that first element becomes init, followed by generalized
///     prefix sum by op. Last element is not considered in prefix sum.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [2, 3, 7];
/// let start = arr.start();
/// let end = arr.end();
/// algo::exclusive_scan_inplace(&mut arr, start, end, 0, |x, y| x + y);
/// assert!(arr.equals(&[0, 2, 5]));
/// ```
pub fn exclusive_scan_inplace<Range, BinaryOp>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    mut init: Range::Element,
    op: BinaryOp,
) where
    Range: OutputRange + ?Sized,
    BinaryOp: Fn(&Range::Element, &Range::Element) -> Range::Element,
{
    while start != end {
        let val = init;
        init = op(&val, &rng.at(&start));
        *rng.at_mut(&start) = val;
        start = rng.after(start);
    }
}

/// Computes the generalized inner product of two ranges using a combination and accumulation operation.
///
/// # Precondition
///   - `[start1, end1)` represents valid positions in rng1.
///
/// # Postcondition
///   - Returns the generalized inner product of rng1 and rng2 using `combine_op` and `reduce_op`.
///   - Complexity: O(n) applications of `combine_op` and `reduce_op`.
///
/// Where n is minimum number of elements in [start1..end1) and [start2..)
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let rng1 = [1, 2, 3];
/// let rng2 = [4, 5, 6];
/// let start1 = rng1.start();
/// let end1 = rng1.end();
/// let start2 = rng2.start();
/// let result = algo::inner_product(&rng1, start1, end1, &rng2, start2, 0, |x, y| x * y, |a, b| a + b);
/// assert_eq!(result, 32); // 0 + (1*4) + (2*5) + (3*6) = 4 + 10 + 18 = 32
/// ```
///```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let rng1 = [1, 2, 3];
/// let rng2 = [4, 5, 6];
/// let start1 = rng1.start();
/// let end1 = rng1.end();
/// let start2 = rng2.start();
/// let result = algo::inner_product(&rng1, start1, end1, &rng2, start2, 0, |x, y| x * y, |a, b| a - b);
/// assert_eq!(result, -32); // 0 - (1*4) - (2*5) -(3*6) = 0 - 4 - 10 - 18 = -32
#[allow(clippy::too_many_arguments)]
pub fn inner_product<Rng1, Rng2, T, U, CombineOp, ReduceOp>(
    rng1: &Rng1,
    mut start1: Rng1::Position,
    end1: Rng1::Position,
    rng2: &Rng2,
    mut start2: Rng2::Position,
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
    while start1 != end1 && !rng2.is_end(&start2) {
        let elem1 = rng1.at(&start1);
        let elem2 = rng2.at(&start2);

        let combined = combine_op(&elem1, &elem2);
        init = reduce_op(init, combined);

        start1 = rng1.after(start1);
        start2 = rng2.after(start2);
    }
    init
}

/// Puts exclusive prefix sum by given operation of src range to dest range, i.e., ith element is
/// not considered in ith prefix sum.
///
/// # Precondition
///   - `[start, end)` represents valid positions in src.
///   - out is valid position in dest.
///   - dest can accomodate n elements.
///
/// # Postcondition
///   - Puts exclusive prefix sum by op of src `[start, end)` to dest starting from out.
///   - Returns the position in out after last element inserted.
///   - Complexity: O(n) applications of op.
///
/// Where n is number of elements in src.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
/// let out = dest.start();
/// let i  =
///   algo::exclusive_scan(&src, src.start(), src.end(), &mut dest, out, 0, |x, y| x + y);
/// assert_eq!(i, 3);
/// assert!(dest.equals(&[0, 1, 3]));
/// ```
pub fn exclusive_scan<SrcRange, DestRange, BinaryOp>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    mut init: DestRange::Element,
    op: BinaryOp,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    BinaryOp: Fn(&DestRange::Element, &SrcRange::Element) -> DestRange::Element,
{
    while start != end {
        let val = init;
        init = op(&val, &src.at(&start));
        start = src.after(start);
        *dest.at_mut(&out) = val;
        out = dest.after(out);
    }
    out
}
