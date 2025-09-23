// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{exec_par, Collection, CollectionExt};

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
    /// assert_eq!(i, Some(2));
    /// ```
    fn parallel_first_position_where<Pred>(
        &self,
        pred: Pred,
    ) -> Option<Self::Position>
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        let hardware_concurrency = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let min_elements_per_core = 5;
        let even_splits = self.splitting_evenly_in_with_min_size(
            hardware_concurrency,
            min_elements_per_core,
        );
        let num_splits = even_splits.len();
        let parallel_tasks = even_splits
            .zip(std::iter::repeat_n(pred, num_splits))
            .map(|(slice, pred)| move || slice.first_position_where(pred));

        exec_par(parallel_tasks).into_iter().flatten().next()
    }
}

impl<R> ParallelCollectionExt for R
where
    R: Collection + ?Sized,
    R::Whole: Send,
{
}
