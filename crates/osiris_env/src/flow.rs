//! Every operation osiris runs, by name: what a log line, a metric or an error is about.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Flow {
    Health,
    AuthConfigsCreate,
    AuthConfigsList,
    AuthConfigsRetrieve,
    AuthConfigsUpdate,
    AuthConfigsDelete,
    ConnectedAccountsCreate,
    ConnectedAccountsList,
    ConnectedAccountsRetrieve,
    ConnectedAccountsDelete,
    ConnectedAccountsRefresh,
    OAuthCallback,
    ActionsRun,
    EventsTrigger,
    TasksRun,
    TriggersCreate,
    TriggersList,
    TriggersRetrieve,
    TriggersDelete,
    TriggerDelivery,
}

impl std::fmt::Display for Flow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
