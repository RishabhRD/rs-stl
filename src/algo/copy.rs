// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

/// Copies elements from src to out of dest if it satisfies predicate.
///
/// # Precondition
///   - `[start, end)` represent valid positions in src.
///   - dest should be able to accomodate all copied elements starting from out.
///
/// # Postcondition
///   - copies elements from `[start, end)` satisfying `pred` to out position of
///     dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n pred applications and maximum n copy operations.
///     where n is number of elements in `[start, end)` of src.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0];
///
/// let out = dest.start();
/// let i = algo::copy_if(&src, src.start(), src.end(), &mut dest, out, |x| x % 2 == 1);
/// assert!(dest.equals(&[1, 3]));
/// assert_eq!(i, 2);
/// ```
pub fn copy_if<SrcRange, DestRange, Pred>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    pred: Pred,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
    Pred: Fn(&SrcRange::Element) -> bool,
{
    while start != end {
        if pred(src.at(&start)) {
            *dest.at_mut(&out) = src.at(&start).clone();
            out = dest.after(out);
        }
        start = src.after(start);
    }
    out
}

/// Copies elements from src to out of dest.
///
/// # Precondition
///   - `[start, end)` represent valid positions in src.
///   - dest should be able to accomodate n elements starting from out where n
///     is number of elements in `[start, end)` of src.
///
/// # Postcondition
///   - copies elements from `[start, end)` to out position of dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations.
///
/// where n is number of elements in `[start, end)`.
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
///
/// let out = dest.start();
/// let i = algo::copy(&src, src.start(), src.end(), &mut dest, out);
/// assert!(dest.equals(&[1, 2, 3]));
/// assert_eq!(i, 3);
/// ```
pub fn copy<SrcRange, DestRange>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    end: SrcRange::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
{
    while start != end {
        *dest.at_mut(&out) = src.at(&start).clone();
        out = dest.after(out);
        start = src.after(start);
    }
    out
}

/// Copies n elements from start of src to out of dest.
///
/// # Precondition
///   - n >= 0.
///   - [start, start + n) represent valid positions in src.
///   - dest should be able to accomodate n elements starting from out
///
/// # Postcondition
///   - copies elements from [start, start + n) to out position of dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
///
/// let out = dest.start();
/// let i = algo::copy_n(&src, src.start(), 2, &mut dest, out);
/// assert!(dest.equals(&[1, 2, 0]));
/// assert_eq!(i, 2);
/// ```
pub fn copy_n<SrcRange, DestRange>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    mut n: usize,
    dest: &mut DestRange,
    mut out: DestRange::Position,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
{
    while n != 0 {
        *dest.at_mut(&out) = src.at(&start).clone();
        out = dest.after(out);
        start = src.after(start);
        n -= 1;
    }
    out
}

/// Copies elements from src to out of dest till the given condition satisfies.
///
/// # Precondition
///   - `[start, till())` represent valid positions in src.
///   - dest should be able to accomodate n elements starting from out where n
///     is number of elements in `[start, end)` of src.
///
/// # Postcondition
///   - copies elements from `[start, till())` to out position of dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations.
///
/// where n is number of elements in `[start, end)`.
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
///
/// let out = dest.start();
/// let i = algo::copy_till(&src, src.start(), |i| *i == src.end(), &mut dest, out);
/// assert!(dest.equals(&[1, 2, 3]));
/// assert_eq!(i, 3);
/// ```
pub fn copy_till<SrcRange, DestRange, TillFn>(
    src: &SrcRange,
    mut start: SrcRange::Position,
    till: TillFn,
    dest: &mut DestRange,
    mut out: DestRange::Position,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
    TillFn: Fn(&SrcRange::Position) -> bool,
{
    // TODO: Add test cases
    while !till(&start) {
        *dest.at_mut(&out) = src.at(&start).clone();
        out = dest.after(out);
        start = src.after(start);
    }
    out
}
