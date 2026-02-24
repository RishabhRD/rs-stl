// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::sync::LazyLock;

use rayon_core::ThreadPool;

use crate::unwrap_option_vec;

/// Returns the global thread pool to execute tasks on.
pub(crate) fn global_thread_pool() -> &'static rayon_core::ThreadPool {
    static POOL: LazyLock<rayon_core::ThreadPool> = LazyLock::new(|| {
        rayon_core::ThreadPoolBuilder::new()
            .build()
            .expect("failed to get global threadpool")
    });
    &POOL
}

/// Executes all task in `tasks` concurrently on global executor.
///
/// # Postcondition
///   - If number of tasks is less than equal to available processors, then
///     tasks would execute parallely.
pub fn exec_par_void_on<Task, Tasks>(mut tasks: Tasks, scheduler: &ThreadPool)
where
    Task: FnOnce() + Send,
    Tasks: Iterator<Item = Task> + Send,
{
    if let Some(first_task) = tasks.next() {
        scheduler.scope(|s| {
            for task in tasks {
                s.spawn(|_| task());
            }
        });
        first_task()
    }
}

/// Executes all task in `tasks` concurrently on global executor and returns
/// the result of each task in order in a vector.
///
/// # Postcondition
///   - If number of tasks is less than equal to available processors, then
///     tasks would execute parallely.
pub fn exec_par_on<Task, TaskResult, Tasks>(
    tasks: Tasks,
    scheduler: &ThreadPool,
) -> Vec<TaskResult>
where
    Task: FnOnce() -> TaskResult + Send,
    Tasks: ExactSizeIterator<Item = Task> + Send,
    TaskResult: Send,
{
    let mut task_results: Vec<Option<TaskResult>> =
        std::iter::repeat_with(|| None).take(tasks.len()).collect();

    let tasks_filling_results = tasks
        .zip(task_results.iter_mut())
        .map(|(task, res)| move || *res = Some(task()));

    exec_par_void_on(tasks_filling_results, scheduler);

    unwrap_option_vec(task_results)
}
