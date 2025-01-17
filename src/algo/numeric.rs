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
    if start == end {
        return;
    }

    std::mem::swap(&mut init, &mut rng.at_mut(&start));
    let mut prev = start.clone();
    start = rng.after(start);

    while start != end {
        init = op(&rng.at(&prev), &init);
        std::mem::swap(&mut init, &mut rng.at_mut(&start));
        prev = start.clone();
        start = rng.after(start);
    }
}
