//! The releases action routes, every one registered.

use super::{
    create_release, delete_release, delete_release_asset, generate_release_notes,
    get_latest_release, get_release, get_release_asset, get_release_by_tag, list_release_assets,
    list_releases, update_release,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(create_release::github_create_release))
        .routes(routes!(delete_release::github_delete_release))
        .routes(routes!(delete_release_asset::github_delete_release_asset))
        .routes(routes!(
            generate_release_notes::github_generate_release_notes
        ))
        .routes(routes!(get_latest_release::github_get_latest_release))
        .routes(routes!(get_release::github_get_release))
        .routes(routes!(get_release_asset::github_get_release_asset))
        .routes(routes!(get_release_by_tag::github_get_release_by_tag))
        .routes(routes!(list_release_assets::github_list_release_assets))
        .routes(routes!(list_releases::github_list_releases))
        .routes(routes!(update_release::github_update_release))
}
