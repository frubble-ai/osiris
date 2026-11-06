//! A refresh task's name and run time, in the form saved on the connection.

use crate::use_cases::lib::connected_accounts::ScheduledRefresh;
use osiris_tasks::Task;

pub fn scheduled(task: &Task) -> ScheduledRefresh {
    ScheduledRefresh {
        name: task.name.clone(),
        at: task.run_at,
    }
}
