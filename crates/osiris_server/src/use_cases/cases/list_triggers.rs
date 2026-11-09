//! Triggers, of one connection or of all, in batches.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_domain::{Cursor, Page, Trigger};

pub struct ListTriggersArgs<'a> {
    pub connected_account_id: Option<&'a str>,
    pub cursor: &'a Cursor,
}

impl UseCases {
    pub async fn list_triggers(&self, args: ListTriggersArgs<'_>) -> UseCaseResult<Page<Trigger>> {
        self.run("list_triggers", async {
            self.context()
                .db
                .list_triggers(args.connected_account_id, args.cursor)
                .await
        })
        .await
    }
}
