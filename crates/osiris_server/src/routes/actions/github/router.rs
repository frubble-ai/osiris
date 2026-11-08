//! GitHub's action routes, every one registered.

use super::{activity, installations, releases, repositories, search, users};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(activity::check_repository_starred::github_check_repository_starred))
        .routes(routes!(activity::list_authenticated_user_events::github_list_authenticated_user_events))
        .routes(routes!(activity::list_authenticated_user_received_events::github_list_authenticated_user_received_events))
        .routes(routes!(activity::list_my_starred_repositories::github_list_my_starred_repositories))
        .routes(routes!(activity::list_public_events::github_list_public_events))
        .routes(routes!(activity::list_repository_events::github_list_repository_events))
        .routes(routes!(activity::list_repository_stargazers::github_list_repository_stargazers))
        .routes(routes!(activity::list_repository_watchers::github_list_repository_watchers))
        .routes(routes!(activity::list_user_public_events::github_list_user_public_events))
        .routes(routes!(activity::list_user_received_public_events::github_list_user_received_public_events))
        .routes(routes!(activity::star_repository::github_star_repository))
        .routes(routes!(activity::unstar_repository::github_unstar_repository))
        .routes(routes!(installations::list_installations::github_list_installations))
        .routes(routes!(installations::list_repositories::github_list_repositories))
        .routes(routes!(releases::create_release::github_create_release))
        .routes(routes!(releases::delete_release::github_delete_release))
        .routes(routes!(releases::delete_release_asset::github_delete_release_asset))
        .routes(routes!(releases::generate_release_notes::github_generate_release_notes))
        .routes(routes!(releases::get_latest_release::github_get_latest_release))
        .routes(routes!(releases::get_release::github_get_release))
        .routes(routes!(releases::get_release_asset::github_get_release_asset))
        .routes(routes!(releases::get_release_by_tag::github_get_release_by_tag))
        .routes(routes!(releases::list_release_assets::github_list_release_assets))
        .routes(routes!(releases::list_releases::github_list_releases))
        .routes(routes!(releases::update_release::github_update_release))
        .routes(routes!(repositories::get_branch::github_get_branch))
        .routes(routes!(repositories::get_repository::github_get_repository))
        .routes(routes!(repositories::list_branches::github_list_branches))
        .routes(routes!(search::search_code::github_search_code))
        .routes(routes!(search::search_commits::github_search_commits))
        .routes(routes!(search::search_labels::github_search_labels))
        .routes(routes!(search::search_repositories::github_search_repositories))
        .routes(routes!(search::search_topics::github_search_topics))
        .routes(routes!(search::search_users::github_search_users))
        .routes(routes!(users::get_current_user::github_get_current_user))
}
