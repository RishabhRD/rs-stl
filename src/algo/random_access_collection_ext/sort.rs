// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, CollectionExt, RandomAccessCollection,
    ReorderableCollection,
};

/// Sorts the collection in place, using the given predicate as comparision between elements.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow strict weak ordering.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are NOT guaranteed to be presevered.
///
/// # Complexity:
///   - O(n * log(n)) worst case where `n == collection.count()`.
pub(crate) fn sort_unstable_by<C, Compare>(
    collection: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool + Clone,
{
    let n = collection.count();
    if n <= 16 {
        insertion_sort(collection, are_in_increasing_order);
    } else {
        let quick_sort_depth = 2 * n.ilog2() as usize;
        if !quick_sort_within(
            collection,
            are_in_increasing_order.clone(),
            quick_sort_depth,
        ) {
            heap_sort(collection, are_in_increasing_order);
        }
    }
}

/// Sorts the collection in place, using the given predicate as comparision between elements.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow strict weak ordering.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are guaranteed to be preserved.
///
/// # Complexity:
///   - O(n * n) where `n == collection.count()`.
pub(crate) fn insertion_sort<C, Compare>(
    collection: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + BidirectionalCollection + ?Sized,
    C::Whole: ReorderableCollection + BidirectionalCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool,
{
    if collection.is_empty() {
        return;
    }

    let mut sorted_end = collection.next_n(collection.start(), 1);
    let end = collection.end();
    while sorted_end != end {
        let mut i = sorted_end.clone();
        loop {
            let j = collection.prior(i.clone());
            if !are_in_increasing_order(&collection.at(&i), &collection.at(&j))
            {
                break;
            }
            collection.swap_at(&i, &j);
            i = j;
            if i == collection.start() {
                break;
            }
        }
        collection.form_next(&mut sorted_end);
    }
}

/// Sorts the collection in place, using the given predicate as comparision between elements
/// using recursion within given depth and returns true. If sorting can't be done  within
/// given depth, returns false.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow strict weak ordering.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are NOT guaranteed to be preserved.
///
/// # Complexity:
///   - O(n * `depth`) where `n == collection.count()`.
pub(crate) fn quick_sort_within<C, Compare>(
    ollection: &mut C,
    are_in_increasing_order: Compare,
    depth: usize,
) -> bool
where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool,
{
    false
}

/// Sorts the collection in place, using the given predicate as comparision between elements.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow strict weak ordering.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are NOT guaranteed to be presevered.
///
/// # Complexity:
///   - O(n * log(n)) worst case where `n == collection.count()`.
pub(crate) fn heap_sort<C, Compare>(
    collection: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool,
{
}
