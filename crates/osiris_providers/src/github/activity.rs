//! Activity: events, stars and watchers.

pub mod check_repository_starred;
pub mod list_authenticated_user_events;
pub mod list_authenticated_user_received_events;
mod list_events;
pub mod list_my_starred_repositories;
pub mod list_public_events;
pub mod list_repository_events;
pub mod list_repository_stargazers;
pub mod list_repository_watchers;
pub mod list_user_public_events;
pub mod list_user_received_public_events;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod star_repository;
pub mod unstar_repository;

pub use check_repository_starred::CheckRepositoryStarred;
pub use list_authenticated_user_events::ListAuthenticatedUserEvents;
pub use list_authenticated_user_received_events::ListAuthenticatedUserReceivedEvents;
pub use list_my_starred_repositories::ListMyStarredRepositories;
pub use list_public_events::ListPublicEvents;
pub use list_repository_events::ListRepositoryEvents;
pub use list_repository_stargazers::ListRepositoryStargazers;
pub use list_repository_watchers::ListRepositoryWatchers;
pub use list_user_public_events::ListUserPublicEvents;
pub use list_user_received_public_events::ListUserReceivedPublicEvents;
pub use star_repository::StarRepository;
pub use unstar_repository::UnstarRepository;

#[cfg(test)]
mod tests;
