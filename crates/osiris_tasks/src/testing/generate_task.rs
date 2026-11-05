//! A task with a random name, due a day from now.

use crate::{ConsentTask, Task, TaskPayload, names};
use fake::Fake;
use time::{Duration, OffsetDateTime};

pub fn generate_task() -> Task {
    let id = format!(
        "ca_{}",
        16.fake::<String>()
            .replace(|character: char| !character.is_ascii_alphanumeric(), "x")
    );
    Task {
        name: names::consent(&id),
        run_at: OffsetDateTime::now_utc() + Duration::days(1),
        payload: TaskPayload::ConsentExpire(ConsentTask {
            tenant_id: format!("org_{}", 8.fake::<String>()),
            connected_account_id: id,
        }),
    }
}
