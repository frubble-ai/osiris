//! The task scheduler, with how long a call to it may take.

use osiris_tasks::TaskScheduler;
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct Scheduling<'a> {
    pub tasks: &'a dyn TaskScheduler,
    pub timeout: Duration,
}
