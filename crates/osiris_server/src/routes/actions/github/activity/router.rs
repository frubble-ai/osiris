//! The activity action routes, every one registered.

use super::{
    check_repository_starred, list_authenticated_user_events,
    list_authenticated_user_received_events, list_my_starred_repositories, list_public_events,
    list_repository_events, list_repository_stargazers, list_repository_watchers,
    list_user_public_events, list_user_received_public_events, star_repository, unstar_repository,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(
            check_repository_starred::github_check_repository_starred
        ))
        .routes(routes!(
            list_authenticated_user_events::github_list_authenticated_user_events
        ))
        .routes(routes!(
            list_authenticated_user_received_events::github_list_authenticated_user_received_events
        ))
        .routes(routes!(
            list_my_starred_repositories::github_list_my_starred_repositories
        ))
        .routes(routes!(list_public_events::github_list_public_events))
        .routes(routes!(
            list_repository_events::github_list_repository_events
        ))
        .routes(routes!(
            list_repository_stargazers::github_list_repository_stargazers
        ))
        .routes(routes!(
            list_repository_watchers::github_list_repository_watchers
        ))
        .routes(routes!(
            list_user_public_events::github_list_user_public_events
        ))
        .routes(routes!(
            list_user_received_public_events::github_list_user_received_public_events
        ))
        .routes(routes!(star_repository::github_star_repository))
        .routes(routes!(unstar_repository::github_unstar_repository))
}
