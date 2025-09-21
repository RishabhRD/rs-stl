// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::unwrap_option_vec;
use std::thread;

/// Executes all task in `tasks` concurrently on global executor.
///
/// # Postcondition
///   - If number of tasks is less than equal to available processors, then
///     tasks would execute parallely.
pub fn exec_par_void<Task, Tasks>(mut tasks: Tasks)
where
    Task: FnOnce() + Send,
    Tasks: Iterator<Item = Task>,
{
    thread::scope(|s| {
        if let Some(first_task) = tasks.next() {
            for task in tasks {
                s.spawn(task);
            }
            first_task()
        }
    });
}

/// Executes all task in `tasks` concurrently on global executor and returns
/// the result of each task in order in a vector.
///
/// # Postcondition
///   - If number of tasks is less than equal to available processors, then
///     tasks would execute parallely.
pub fn exec_par<Task, TaskResult, Tasks>(tasks: Tasks) -> Vec<TaskResult>
where
    Task: FnOnce() -> TaskResult + Send,
    Tasks: ExactSizeIterator<Item = Task>,
    TaskResult: Send,
{
    let mut task_results: Vec<Option<TaskResult>> =
        std::iter::repeat_with(|| None).take(tasks.len()).collect();

    let tasks_filling_results = tasks
        .zip(task_results.iter_mut())
        .map(|(task, res)| move || *res = Some(task()));

    exec_par_void(tasks_filling_results);

    unwrap_option_vec(task_results)
}
