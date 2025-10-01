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
        let min_elements_per_core = 512;
        let even_splits = self.splitting_evenly_in_with_min_size(
            hardware_concurrency,
            min_elements_per_core,
        );
        let num_splits = even_splits.len();
        let parallel_tasks = even_splits
            .zip(std::iter::repeat_n(pred, num_splits))
            .map(|(slice, pred)| move || slice.first_position_where(pred));

        // TODO: implement cancellation.
        exec_par(parallel_tasks).into_iter().flatten().next()
    }

    /// Finds position of first element in `self` equals `e`. If no such element
    /// exists, returns `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 3];
    /// let i = arr.parallel_first_position_of(&3);
    /// assert_eq!(i, Some(2));
    /// ```
    fn parallel_first_position_of(
        &self,
        e: &Self::Element,
    ) -> Option<Self::Position>
    where
        Self::Element: Eq + Sync, // TODO: is Sync really necessary??
    {
        self.parallel_first_position_where(|x| x == e)
    }

    /// Finds position of last element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4];
    /// let i = arr.parallel_last_position_where(|x| x % 2 == 1);
    /// assert_eq!(i, Some(2));
    /// ```
    fn parallel_last_position_where<Pred>(
        &self,
        pred: Pred,
    ) -> Option<Self::Position>
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        let hardware_concurrency = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let min_elements_per_core = 512;
        let even_splits = self.splitting_evenly_in_with_min_size(
            hardware_concurrency,
            min_elements_per_core,
        );
        let num_splits = even_splits.len();
        let parallel_tasks = even_splits
            .zip(std::iter::repeat_n(pred, num_splits))
            .map(|(slice, pred)| move || slice.last_position_where(pred));

        // TODO: implement cancellation.
        exec_par(parallel_tasks).into_iter().flatten().last()
    }

    /// Finds position of `last` element equals `e`. If no such element exist,
    /// return `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 3, 3];
    /// let i = arr.parallel_last_position_of(&3);
    /// assert_eq!(i, Some(2));
    /// ```
    fn parallel_last_position_of(
        &self,
        e: &Self::Element,
    ) -> Option<Self::Position>
    where
        Self::Element: Eq + Sync,
    {
        self.parallel_last_position_where(|x| x == e)
    }

    /*-----------------Predicate Test Algorithms-----------------*/

    /// Returns true iff all elements in `self` satisfies `pred`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 3, 5];
    /// assert!(arr.parallel_all_satisfy(|x| x % 2 == 1));
    /// ```
    fn parallel_all_satisfy<Pred>(&self, pred: Pred) -> bool
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        let hardware_concurrency = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let min_elements_per_core = 512;
        let even_splits = self.splitting_evenly_in_with_min_size(
            hardware_concurrency,
            min_elements_per_core,
        );
        let num_splits = even_splits.len();
        let parallel_tasks = even_splits
            .zip(std::iter::repeat_n(pred, num_splits))
            .map(|(slice, pred)| move || slice.all_satisfy(pred));

        // TODO: implement cancellation.
        exec_par(parallel_tasks).into_iter().all(|e| e)
    }

    /// Returns true iff atleast one element in `self` satisfies `pred`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 5];
    /// assert!(arr.any_satisfy(|x| x % 2 == 1));
    /// ```
    fn parallel_any_satisfy<Pred>(&self, pred: Pred) -> bool
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        let hardware_concurrency = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let min_elements_per_core = 512;
        let even_splits = self.splitting_evenly_in_with_min_size(
            hardware_concurrency,
            min_elements_per_core,
        );
        let num_splits = even_splits.len();
        let parallel_tasks = even_splits
            .zip(std::iter::repeat_n(pred, num_splits))
            .map(|(slice, pred)| move || slice.any_satisfy(pred));

        // TODO: implement cancellation.
        exec_par(parallel_tasks).into_iter().any(|e| e)
    }

    /// Returns true iff no element in `self` satisfies `pred`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [2, 4, 6];
    /// assert!(arr.none_satisfy(|x| x % 2 == 1));
    /// ```
    fn parallel_none_satisfy<Pred>(&self, pred: Pred) -> bool
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        let hardware_concurrency = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let min_elements_per_core = 512;
        let even_splits = self.splitting_evenly_in_with_min_size(
            hardware_concurrency,
            min_elements_per_core,
        );
        let num_splits = even_splits.len();
        let parallel_tasks = even_splits
            .zip(std::iter::repeat_n(pred, num_splits))
            .map(|(slice, pred)| move || slice.none_satisfy(pred));

        // TODO: implement cancellation.
        exec_par(parallel_tasks).into_iter().all(|e| e)
    }
}

impl<R> ParallelCollectionExt for R
where
    R: Collection + ?Sized,
    R::Whole: Send,
{
}
