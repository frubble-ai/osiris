//! A scheduler that needs nothing arranged, offered to the scheduler contract.

use crate::{TaskScheduler, testing::SchedulerHarness};

pub struct MockSchedulerHarness<S>(pub S);

impl<S: TaskScheduler> SchedulerHarness for MockSchedulerHarness<S> {
    fn scheduler(&self) -> &dyn TaskScheduler {
        &self.0
    }
}
