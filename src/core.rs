// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// Equality Comparable + Movable + Destructable.
///
/// As per Stepanov (not exact), Types can be:
///   - compared for equality
///   - moved
///   - destructed
pub trait SemiRegular: Eq {}
impl<T> SemiRegular for T where T: Eq {}

/// SemiRegular + Clonable
///
/// As per Stepanov (not exact), Types can be:
///   - SemiRegular
///   - cloned
pub trait Regular: SemiRegular + Clone {}
impl<T> Regular for T where T: SemiRegular + Clone {}

/// Models a single-pass range.
///
/// This is most primitive range trait. It specfies requirement for supporting
/// single pass iteration.
///
/// - type Element: for defining the type of element in range.
/// - type Position: for defining the type of position/index in range.
/// - start() would return the first position
/// - end() would return the last position.
/// - For traversing forward after(i) would return next position after i.
/// - Accessing any element immutably can be done using at(i)
/// - after_n is additional convinient method supported to get to nth position after current
///   position.
/// - Position type is bounded to SemiRegular so that algorithms working on
///   InputRange doesn't accidently copy any position.
pub trait InputRange {
    /// Type of the element contained in self
    type Element;

    /// Type of the positions in self
    type Position: SemiRegular;

    /// Type of reference to element.
    ///
    /// Very useful in scenarios when range actually doesn't contain the elements
    /// and proxy reference is necessary.
    type ElementRef<'a>: std::ops::Deref<Target = Self::Element>
    where
        Self: 'a;

    /// Returns the position of first element in self,
    /// or if self is empty then start() == end()
    fn start(&self) -> Self::Position;

    /// Returns true if i is the end position of range.
    fn is_end(&self, i: &Self::Position) -> bool;

    /// Returns position immediately after i
    ///
    /// # Precondition
    ///   - i != end()
    fn after(&self, i: Self::Position) -> Self::Position;

    /// Returns nth position after i.
    ///
    /// # Precondition
    ///   - There are n valid positions in self after i.
    ///
    /// # Complexity
    /// n applications of after().
    fn after_n(&self, mut i: Self::Position, mut n: usize) -> Self::Position {
        while n > 0 {
            i = self.after(i);
            n -= 1;
        }
        i
    }

    /// Access element at position i.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    /// O(1)
    fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a>;
}

pub trait Collection<'a>:
    InputRange<ElementRef<'a> = &'a <Self as InputRange>::Element>
where
    Self: 'a,
{
}

/// Models a bounded range, i.e., the range whose end position is known.
pub trait BoundedRange: InputRange {
    /// Returns position immediately after position of last element.
    fn end(&self) -> Self::Position;
}

/// Models a multi-pass range.
///
/// - ForwardRange is an extension of InputRange.
/// - ForwardRange models multi-pass range by allowing Position to be Regular.
///   This allows cloning position object, so that clone can be used for
///   another iteration at any point.
/// - Usually any algorithm, that needs to store iterator usually have minimum
///   ForwardRange requirement.
/// - Supports a convenient distance function, to get distance between 2 positions.
pub trait ForwardRange: InputRange<Position: Regular> {
    /// Returns distance between position `[from, to)`.
    ///
    /// # Precondition
    ///   - `[from, to)` represents valid position in range.
    ///
    /// # Complexity
    /// O(n)
    fn distance(&self, mut from: Self::Position, to: Self::Position) -> usize {
        let mut res = 0;
        while from != to {
            from = self.after(from);
            res += 1;
        }
        res
    }
}

/// Models a bidirectional range, which can be traversed forward as well as backward.
///
/// - BidirectionalRange is an extension of ForwardRange.
/// - For supporting backward iteration, before(i) function is supported. before(i)
///   function returns the position just before current position.
/// - A before_n convenient function is there to get nth position before current position.
pub trait BidirectionalRange: ForwardRange {
    /// Returns position immediately before i
    ///
    /// # Precondition
    ///   - i != start()
    fn before(&self, i: Self::Position) -> Self::Position;

    /// Returns nth position before i
    ///
    /// # Precondition
    ///   - self has n valid positions before i.
    ///
    /// # Complexity
    /// n applications of before.
    fn before_n(&self, mut i: Self::Position, mut n: usize) -> Self::Position {
        while n > 0 {
            i = self.before(i);
            n -= 1;
        }
        i
    }
}

/// Models a random access range (similar to array) where jumping to any position from any
/// other position is O(1) operation.
///
/// - RandomAccessRange is extension of BidirectionalRange.
/// - RandomAccessRange enforces the Position of range should be ordered.
/// - RandomAccessRange doesn't add any new method but introduces complexity
///   requirements mentioned below. The complexity requirements ensure that
///   any position jump is O(1).
///
/// # Complexity Requirements
///   - `rng.distance(from, to)` -> O(1).
///   - `rng.after_n(i)` -> O(1).
///   - `rng.before_n(i)` -> O(1).
///
///   NOTE: If complexity requirements are not formed any algorithm on RandomAccessRange
///   have undefined behavior.
pub trait RandomAccessRange:
    BidirectionalRange<Position: Regular + Ord>
{
}

/// Models mutable range that can only be reordered.
///
/// - SemiOutputRange is extension of ForwardRange.
/// - SemiOutputRange models primitive mutable range that just supports reordering
///   of its elements.
/// - For supporting reordering of elements, `swap_at` function is required.
///   It allows to swap 2 elements at given positions.
pub trait SemiOutputRange: ForwardRange {
    /// Swap elements at position i and j
    ///
    /// # Precondition
    ///   - i and j is a valid position in self.
    ///
    /// # Complexity Requirement
    /// O(1)
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);
}

/// Models a fully mutable range.
///
/// OutputRange is extension of SemiOutputRange.
///
/// There are 2 types of safe mutation possible for ranges:
/// 1. Reordering of elements that is provided by SemiOutputRange.
/// 2. Element Assignment: Assign different value to element at any position.
///    For the same, `at_mut` function is required that returns mutable
///    reference to element at any position.
pub trait OutputRange: SemiOutputRange {
    /// Type of mutable reference to element.
    ///
    /// Very useful in scenarios when range actually doesn't contain the elements
    /// and proxy mutable reference is necessary.
    type ElementMutRef<'a>: std::ops::DerefMut<Target = Self::Element>
    where
        Self: 'a;

    /// Access element at position i
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    /// O(1)
    fn at_mut<'a>(&'a mut self, i: &Self::Position) -> Self::ElementMutRef<'a>;
}

/// Marker trait for a view.
///
/// A view is also a range. However, unlike `Vec<T>`, view is intended for used
/// as non-owning range i.e., it can be one of following:
/// 1. refers to another range.
/// 2. owns another view.
/// 3. doesn't actually contains any element.
pub trait View: InputRange {}
