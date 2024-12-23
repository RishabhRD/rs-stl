// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, RandomAccessRange, SemiOutputRange};

/// Finds largest range that from start represents a heap wrt comparator.
///
/// # Precondition
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns last position `i` in rng such that
///     rng at `[rng.start(), i)` is a heap with respect to cmp.
///   - Complexity: O(n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// Where n is number of elements in rng.
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2, 6];
///
/// let i = rng::is_heap_until_by(&arr, |x, y| x < y);
/// assert_eq!(i, 7);
///
/// let i = arr.is_heap_until_by(|x, y| x < y);
/// assert_eq!(i, 7);
/// ```
pub fn is_heap_until_by<Range, Compare>(
    rng: &Range,
    cmp: Compare,
) -> Range::Position
where
    Range: RandomAccessRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::is_heap_until_by(rng, rng.start(), rng.end(), cmp)
}

/// Finds largest range that from start represents a heap.
///
/// # Precondition
///   - `[start, end)` reperesents valid position in rng.
///
/// # Postcondition
///   - Returns last position `i` in `[start, end)` of rng such that
///     rng at `[start, i)` is a heap.
///   - Complexity: O(n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// Where n is number of elements in `[start, end)`.
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2, 6];
///
/// let i = rng::is_heap_until(&arr);
/// assert_eq!(i, 7);
///
/// let i = arr.is_heap_until();
/// assert_eq!(i, 7);
/// ```
pub fn is_heap_until<Range>(rng: &Range) -> Range::Position
where
    Range: RandomAccessRange + ?Sized,
    Range::Element: Ord,
{
    algo::is_heap_until(rng, rng.start(), rng.end())
}

/// Returns true if given range is a heap wrt comparator.
///
/// # Precondition
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns true if rng is a heap wrt cmp. Otherwise, returns false.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2];
/// assert!(rng::is_heap_by(&arr, |x, y| x < y));
/// assert!(arr.is_heap_by(|x, y| x < y));
/// ```
pub fn is_heap_by<Range, Compare>(rng: &Range, cmp: Compare) -> bool
where
    Range: RandomAccessRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::is_heap_by(rng, rng.start(), rng.end(), cmp)
}

/// Returns true if given range is a heap.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if rng is a heap. Otherwise, returns false.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2];
///
/// assert!(rng::is_heap(&arr));
/// assert!(arr.is_heap());
/// ```
pub fn is_heap<Range>(rng: &Range) -> bool
where
    Range: RandomAccessRange + ?Sized,
    Range::Element: Ord,
{
    algo::is_heap(rng, rng.start(), rng.end())
}

/// Inserts last element in specified range into a heap wrt cmp. After insertion, full range would
/// be heap wrt cmp.
///
/// # Precondition
///   - rng is empty || `[rng.start(), rng.end() - 1)` should be a heap wrt cmp.
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Inserts element at `rng.end() - 1` to heap at `[rng.start(), rng.end())`.
///     After operation, full rng will be a heap wrt cmp.
///   - Complexity: O(log n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [8, 2, 9];
/// let start = arr.start();
/// let end = arr.end();
/// rng::push_heap_by(&mut arr, |x, y| x < y);
/// assert!(arr.is_heap_by(|x, y| x < y));
///
/// let mut arr = [8, 2, 9];
/// let start = arr.start();
/// let end = arr.end();
/// arr.push_heap_by(|x, y| x < y);
/// assert!(arr.is_heap_by(|x, y| x < y));
/// ```
pub fn push_heap_by<Range, Compare>(rng: &mut Range, cmp: Compare)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let start = rng.start();
    let end = rng.end();
    algo::push_heap_by(rng, start, end, cmp);
}

/// Inserts last element in specified range into a heap. After insertion, full range would be a
/// heap.
///
/// # Precondition
///   - rng is empty || `[rng.start(), rng.end() - 1)` should be a heap.
///
/// # Postcondition
///   - Inserts element at `rng.end() - 1` to heap at `[rng.start(), rng.end())`.
///     After operation, full rng will be a heap.
///   - Complexity: O(log n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [8, 2, 9];
/// let start = arr.start();
/// let end = arr.end();
/// rng::push_heap(&mut arr);
/// assert!(arr.is_heap());
///
/// let mut arr = [8, 2, 9];
/// let start = arr.start();
/// let end = arr.end();
/// arr.push_heap();
/// assert!(arr.is_heap());
/// ```
pub fn push_heap<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    push_heap_by(rng, |x, y| x < y)
}

/// Swaps element at `rng.start()` position with element before `rng.end()` position and ensures `[rng.start(), rng.end() - 1)` is a heap wrt cmp.
///
/// # Precondition
///   - rng is a heap wrt cmp.
///   - cmp should follow strict-weak-ordering relationship.
///
/// # Postcondition
///   - Swaps element at `rng.start()` position with element before `rng.end()` position
///     and then ensures `[rng.start(), rng.end() - 1)` is a heap wrt cmp.
///   - If rng is empty, then do nothing.
///   - Complexity: O(log n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [9, 8, 7];
/// rng::pop_heap_by(&mut arr, |x, y| x < y);
/// assert!(&arr[0..2].is_heap_by(|x, y| x < y));
/// assert!(arr.equals(&[8, 7, 9]));
///
/// let mut arr = [9, 8, 7];
/// arr.pop_heap_by(|x, y| x < y);
/// assert!(&arr[0..2].is_heap_by(|x, y| x < y));
/// assert!(arr.equals(&[8, 7, 9]));
/// ```
pub fn pop_heap_by<Range, Compare>(rng: &mut Range, cmp: Compare)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let start = rng.start();
    let end = rng.end();
    algo::pop_heap_by(rng, start, end, cmp);
}

/// Swaps element at `rng.start()` position with element before `rng.end()` position and ensures `[rng.start(), rng.end() - 1)` is a heap.
///
/// # Precondition
///   - rng is a heap.
///
/// # Postcondition
///   - Swaps element at `rng.start()` position with element before `rng.end()` position
///     and then ensures `[rng.start(), rng.end() - 1)` is a heap.
///   - If rng is empty, then do nothing.
///   - Complexity: O(log n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [9, 8, 7];
/// rng::pop_heap(&mut arr);
/// assert!(&arr[0..2].is_heap());
/// assert!(arr.equals(&[8, 7, 9]));
///
/// let mut arr = [9, 8, 7];
/// arr.pop_heap();
/// assert!(&arr[0..2].is_heap());
/// assert!(arr.equals(&[8, 7, 9]));
/// ```
pub fn pop_heap<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::pop_heap(rng, start, end);
}

/// Reorders the range such that resulting range is heap wrt cmp.
///
/// # Precondition
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Reorders rng such that resulting range at is a heap wrt cmp.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [2, 5, 1, 4];
/// rng::make_heap_by(&mut arr, |x, y| x < y);
/// assert!(arr.is_heap_by(|x, y| x < y));
///
/// let mut arr = [2, 5, 1, 4];
/// arr.make_heap_by(|x, y| x < y);
/// assert!(arr.is_heap_by(|x, y| x < y));
/// ```
pub fn make_heap_by<Range, Compare>(rng: &mut Range, cmp: Compare)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::make_heap_by(rng, start, end, cmp);
}

/// Reorders the range such that resulting range is heap.
///
/// # Precondition
///
/// # Postcondition
///   - Reorders rng such that resulting range is a heap.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [2, 5, 1, 4];
/// rng::make_heap(&mut arr);
/// assert!(arr.is_heap());
///
/// let mut arr = [2, 5, 1, 4];
/// arr.make_heap();
/// assert!(arr.is_heap());
/// ```
pub fn make_heap<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::make_heap(rng, start, end);
}

pub mod infix {
    use crate::{rng, RandomAccessRange, SemiOutputRange};

    /// `is_heap_until`, `is_heap_until_by`.
    pub trait STLHeapExt: RandomAccessRange {
        fn is_heap_until_by<Compare>(&self, cmp: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn is_heap_until(&self) -> Self::Position
        where
            Self::Element: Ord;

        fn is_heap_by<Compare>(&self, cmp: Compare) -> bool
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn is_heap(&self) -> bool
        where
            Self::Element: Ord;
    }

    impl<R> STLHeapExt for R
    where
        R: RandomAccessRange + ?Sized,
    {
        fn is_heap_until_by<Compare>(&self, cmp: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::is_heap_until_by(self, cmp)
        }

        fn is_heap_until(&self) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::is_heap_until(self)
        }

        fn is_heap_by<Compare>(&self, cmp: Compare) -> bool
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::is_heap_by(self, cmp)
        }

        fn is_heap(&self) -> bool
        where
            Self::Element: Ord,
        {
            rng::is_heap(self)
        }
    }

    /// `push_heap`, `push_heap_by`.
    pub trait STLOutputHeapExt: RandomAccessRange + SemiOutputRange {
        fn push_heap_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn push_heap(&mut self)
        where
            Self::Element: Ord;

        fn pop_heap_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn pop_heap(&mut self)
        where
            Self::Element: Ord;

        fn make_heap_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone;

        fn make_heap(&mut self)
        where
            Self::Element: Ord;
    }

    impl<R> STLOutputHeapExt for R
    where
        R: RandomAccessRange + SemiOutputRange + ?Sized,
    {
        fn push_heap_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::push_heap_by(self, cmp);
        }

        fn push_heap(&mut self)
        where
            Self::Element: Ord,
        {
            rng::push_heap(self);
        }

        fn pop_heap_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::pop_heap_by(self, cmp);
        }

        fn pop_heap(&mut self)
        where
            Self::Element: Ord,
        {
            rng::pop_heap(self);
        }

        fn make_heap_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::make_heap_by(self, cmp);
        }

        fn make_heap(&mut self)
        where
            Self::Element: Ord,
        {
            rng::make_heap(self);
        }
    }
}
