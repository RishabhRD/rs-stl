// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use rayon_core::ThreadPool;

use crate::{exec_par_on, global_thread_pool, Collection, CollectionExt};

/// Parallel Algorithms for `Collection`.
pub trait ParallelCollectionExt: Collection {
    /*-----------------Find Algorithms-----------------*/

    /// Finds position of first element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// Parallel version of `first_position_where` executing on `scheduler`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn first_position_where_par_on<Pred>(
        &self,
        scheduler: &ThreadPool,
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
        exec_par_on(parallel_tasks, scheduler)
            .into_iter()
            .flatten()
            .next()
    }

    /// Finds position of first element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// Parallel version of `first_position_where`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn first_position_where_par<Pred>(
        &self,
        pred: Pred,
    ) -> Option<Self::Position>
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        self.first_position_where_par_on(global_thread_pool(), pred)
    }

    /// Finds position of first element in `self` equals `e`. If no such element
    /// exists, returns `self.end()`.
    ///
    /// Parallel version of `first_position_of` executing on `scheduler`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn first_position_of_par_on(
        &self,
        scheduler: &ThreadPool,
        e: &Self::Element,
    ) -> Option<Self::Position>
    where
        Self::Element: Eq + Sync, // TODO: is Sync really necessary??
    {
        self.first_position_where_par_on(scheduler, |x| x == e)
    }

    /// Finds position of first element in `self` equals `e`. If no such element
    /// exists, returns `self.end()`.
    ///
    /// Parallel version of `first_position_of`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn first_position_of_par(&self, e: &Self::Element) -> Option<Self::Position>
    where
        Self::Element: Eq + Sync, // TODO: is Sync really necessary??
    {
        self.first_position_of_par_on(global_thread_pool(), e)
    }

    /// Finds position of last element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// Parallel version of `last_position_where` executing on `scheduler`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    /// ```
    fn last_position_where_par_on<Pred>(
        &self,
        scheduler: &ThreadPool,
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
        exec_par_on(parallel_tasks, scheduler)
            .into_iter()
            .flatten()
            .last()
    }

    /// Finds position of last element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// Parallel version of `last_position_where`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    /// ```
    fn last_position_where_par<Pred>(
        &self,
        pred: Pred,
    ) -> Option<Self::Position>
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        self.last_position_where_par_on(global_thread_pool(), pred)
    }

    /// Finds position of `last` element equals `e`. If no such element exist,
    /// return `self.end()`.
    ///
    /// Parallel version of `last_position_of` executing on `scheduler`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn last_position_of_par_on(
        &self,
        scheduler: &ThreadPool,
        e: &Self::Element,
    ) -> Option<Self::Position>
    where
        Self::Element: Eq + Sync,
    {
        self.last_position_where_par_on(scheduler, |x| x == e)
    }

    /// Finds position of `last` element equals `e`. If no such element exist,
    /// return `self.end()`.
    ///
    /// Parallel version of `last_position_of`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn last_position_of_par(&self, e: &Self::Element) -> Option<Self::Position>
    where
        Self::Element: Eq + Sync,
    {
        self.last_position_of_par_on(global_thread_pool(), e)
    }

    /*-----------------Predicate Test Algorithms-----------------*/

    /// Returns true iff all elements in `self` satisfies `pred`.
    ///
    /// Parallel version of `all_satisfy` executing on `scheduler`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn all_satisfy_par_on<Pred>(
        &self,
        scheduler: &ThreadPool,
        pred: Pred,
    ) -> bool
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
        exec_par_on(parallel_tasks, scheduler)
            .into_iter()
            .all(|e| e)
    }

    /// Returns true iff all elements in `self` satisfies `pred`.
    ///
    /// Parallel version of `all_satisfy`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn all_satisfy_par<Pred>(&self, pred: Pred) -> bool
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        self.all_satisfy_par_on(global_thread_pool(), pred)
    }

    /// Returns true iff atleast one element in `self` satisfies `pred`.
    ///
    /// Parallel version of `any_satisfy` executing on `scheduler`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn any_satisfy_par_on<Pred>(
        &self,
        scheduler: &ThreadPool,
        pred: Pred,
    ) -> bool
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
        exec_par_on(parallel_tasks, scheduler)
            .into_iter()
            .any(|e| e)
    }

    /// Returns true iff atleast one element in `self` satisfies `pred`.
    ///
    /// Parallel version of `any_satisfy`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn any_satisfy_par<Pred>(&self, pred: Pred) -> bool
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        self.any_satisfy_par_on(global_thread_pool(), pred)
    }

    /// Returns true iff no element in `self` satisfies `pred`.
    ///
    /// Parallel version of `none_satisfy` executing on `scheduler`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn none_satisfy_par_on<Pred>(
        &self,
        scheduler: &ThreadPool,
        pred: Pred,
    ) -> bool
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
        exec_par_on(parallel_tasks, scheduler)
            .into_iter()
            .all(|e| e)
    }

    /// Returns true iff no element in `self` satisfies `pred`.
    ///
    /// Parallel version of `none_satisfy`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    fn none_satisfy_par<Pred>(&self, pred: Pred) -> bool
    where
        Pred: Fn(&Self::Element) -> bool + Clone + Send,
    {
        self.none_satisfy_par_on(global_thread_pool(), pred)
    }
}

impl<R> ParallelCollectionExt for R where R: Collection + ?Sized {}
