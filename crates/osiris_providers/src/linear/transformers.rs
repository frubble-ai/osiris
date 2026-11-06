//! Linear's GraphQL answers as it sends them, and into our types.

use super::types::LinearTeam;
use serde::Deserialize;

/// Any GraphQL answer: `data`, or `errors` beside a 200.
#[derive(Deserialize)]
pub struct GraphqlAnswer<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphqlError>>,
}

#[derive(Deserialize)]
pub struct GraphqlError {
    pub message: String,
    pub extensions: Option<GraphqlErrorExtensions>,
}

#[derive(Deserialize)]
pub struct GraphqlErrorExtensions {
    pub code: Option<String>,
}

/// `teams` with its page info.
#[derive(Deserialize)]
pub struct TeamsData {
    pub teams: TeamsConnection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamsConnection {
    nodes: Vec<TeamAnswer>,
    pub page_info: PageInfo,
}

#[derive(Deserialize)]
struct TeamAnswer {
    id: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

impl TeamsConnection {
    pub fn teams(self) -> impl Iterator<Item = LinearTeam> {
        self.nodes.into_iter().map(|team| LinearTeam {
            id: team.id,
            name: team.name,
        })
    }
}
