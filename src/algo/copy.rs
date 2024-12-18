// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

/// Copies elements from src to out of dest if it satisfies predicate.
///
/// Precondition:
///   - `[start, end)` represent valid positions in src.
///   - dest should be able to accomodate all copied elements starting from out.
///
/// Postcondition:
///   - copies elements from `[start, end)` satisfying `pred` to out position of
///     dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n pred applications and maximum n copy operations.
///     where n is number of elements in `[start, end)` of src.
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
/// Precondition:
///   - `[start, end)` represent valid positions in src.
///   - dest should be able to accomodate n elements starting from out where n
///     is number of elements in `[start, end)` of src.
///
/// Postcondition:
///   - copies elements from `[start, end)` to out position of dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations.
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
/// Precondition:
///   - n >= 0.
///   - [start, start + n) represent valid positions in src.
///   - dest should be able to accomodate n elements starting from out
///
/// Postcondition:
///   - copies elements from [start, start + n) to out position of dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations.
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
