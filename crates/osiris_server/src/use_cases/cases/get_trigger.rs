//! One trigger.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_domain::Trigger;

pub struct GetTriggerArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn get_trigger(&self, args: GetTriggerArgs<'_>) -> UseCaseResult<Trigger> {
        self.run("get_trigger", async {
            self.context().db.trigger(args.id).await
        })
        .await
    }
}
