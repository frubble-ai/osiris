//! A scheduler that keeps tasks in memory, so tests can inspect them and run them by hand.

use crate::{Scheduled, Task, TaskError, TaskScheduler};
use async_trait::async_trait;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Mutex, PoisonError},
};

#[derive(Default)]
pub struct MemoryScheduler {
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    pending: BTreeMap<String, Task>,
    /// Every name ever scheduled. Like a real scheduler, this one never reuses a name.
    used: BTreeSet<String>,
    cancelled: Vec<String>,
    failing: bool,
}

impl MemoryScheduler {
    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The tasks waiting to run, ordered by name.
    pub fn pending(&self) -> Vec<Task> {
        self.state().pending.values().cloned().collect()
    }

    /// The waiting task called `name`, if there is one.
    pub fn get(&self, name: &str) -> Option<Task> {
        self.state().pending.get(name).cloned()
    }

    /// Removes a waiting task and returns it, the way a scheduler does when it sends the task to
    /// the worker.
    pub fn take(&self, name: &str) -> Option<Task> {
        self.state().pending.remove(name)
    }

    /// The names of the tasks cancelled so far, oldest first.
    pub fn cancelled(&self) -> Vec<String> {
        self.state().cancelled.clone()
    }

    /// Makes every call fail (or work again), to simulate a scheduler that cannot be reached.
    pub fn fail(&self, failing: bool) {
        self.state().failing = failing;
    }
}

#[async_trait]
impl TaskScheduler for MemoryScheduler {
    async fn schedule(&self, task: &Task) -> Result<Scheduled, TaskError> {
        let mut state = self.state();
        if state.failing {
            return Err(TaskError::new("unavailable"));
        }
        if !state.used.insert(task.name.clone()) {
            return Ok(Scheduled::AlreadyScheduled);
        }
        state.pending.insert(task.name.clone(), task.clone());
        Ok(Scheduled::Created)
    }

    async fn cancel(&self, name: &str) -> Result<(), TaskError> {
        let mut state = self.state();
        if state.failing {
            return Err(TaskError::new("unavailable"));
        }
        state.pending.remove(name);
        state.cancelled.push(name.to_owned());
        Ok(())
    }
}
