// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, CollectionExt, RandomAccessCollection,
    ReorderableCollection, ReorderableCollectionExt,
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
    collection: &mut C,
    are_in_increasing_order: Compare,
    depth: usize,
) -> bool
where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool + Clone,
{
    if collection.start() == collection.end()
        || collection.next(collection.start()) == collection.end()
    {
        return true;
    }

    if depth == 0 {
        return false;
    }

    let start = collection.start();

    // Partition collection except first element.
    let rest_start = collection.next(start.clone());
    let (pivot, mut rest) = collection.full_mut().split_at_mut(rest_start);
    let p = rest.partition(|e| !are_in_increasing_order(e, &pivot.at(&start)));

    // Fix posiiton of first element.
    let partition_point = collection.prior(p);
    collection.swap_at(&start, &partition_point);

    // Quick sort both parts.
    let left = quick_sort_within(
        &mut collection.prefix_upto_mut(partition_point.clone()),
        are_in_increasing_order.clone(),
        depth - 1,
    );

    let right = quick_sort_within(
        &mut collection.suffix_from_mut(partition_point),
        are_in_increasing_order,
        depth - 1,
    );

    left && right
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
