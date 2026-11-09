//! A trigger's scheduled check number `check`, due `interval` from now and named after its
//! number.

use osiris_tasks::{Task, TaskPayload, TriggerTask, names};
use time::{Duration, OffsetDateTime};

/// The `trigger.poll` task for `trigger_id`'s check number `check`.
pub fn next_trigger_check(
    trigger_id: &str,
    connected_account_id: &str,
    check: i64,
    interval: Duration,
) -> Task {
    Task {
        name: names::trigger_poll(trigger_id, check),
        run_at: OffsetDateTime::now_utc().saturating_add(interval),
        payload: TaskPayload::TriggerPoll(TriggerTask {
            trigger_id: trigger_id.to_owned(),
            connected_account_id: connected_account_id.to_owned(),
            check,
        }),
    }
}
