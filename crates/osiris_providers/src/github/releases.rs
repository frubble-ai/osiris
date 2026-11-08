//! Releases and their assets.

pub mod create_release;
pub mod delete_release;
pub mod delete_release_asset;
pub mod generate_release_notes;
pub mod get_latest_release;
pub mod get_release;
pub mod get_release_asset;
pub mod get_release_by_tag;
pub mod list_release_assets;
pub mod list_releases;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod update_release;

pub use create_release::CreateRelease;
pub use delete_release::DeleteRelease;
pub use delete_release_asset::DeleteReleaseAsset;
pub use generate_release_notes::GenerateReleaseNotes;
pub use get_latest_release::GetLatestRelease;
pub use get_release::GetRelease;
pub use get_release_asset::GetReleaseAsset;
pub use get_release_by_tag::GetReleaseByTag;
pub use list_release_assets::ListReleaseAssets;
pub use list_releases::ListReleases;
pub use update_release::UpdateRelease;

#[cfg(test)]
mod tests;
