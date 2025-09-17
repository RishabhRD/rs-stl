// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, CollectionExt};
use std::thread;

/// Parallel Algorithms for `Collection`.
pub trait ParallelCollectionExt: Collection
where
    Self::Whole: Send,
{
    /*-----------------Find Algorithms-----------------*/

    /// Finds position of first element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let i = arr.parallel_first_position_where(|x| *x == 3);
    /// assert_eq!(i, 2);
    /// ```
    fn parallel_first_position_where<Pred>(&self, pred: Pred) -> Self::Position
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        let hardware_concurrency = 2;
        let min_elements_for_core = 5;

        thread::scope(|scope| {
            let handles: Vec<_> = self
                .split_evenly_in(hardware_concurrency, min_elements_for_core)
                .map(|s| {
                    let pred = pred.clone();
                    scope.spawn(move || {
                        let p = s.first_position_where(pred);
                        if p == s.end() {
                            None
                        } else {
                            Some(p)
                        }
                    })
                })
                .collect();

            let res =
                handles.into_iter().filter_map(|h| h.join().unwrap()).next();

            res.unwrap_or_else(|| self.end())
        })
    }
}

impl<R> ParallelCollectionExt for R
where
    R: Collection + ?Sized,
    R::Whole: Send,
{
}
