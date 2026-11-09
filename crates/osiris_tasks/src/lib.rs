//! Scheduled tasks: work osiris must do at a set time rather than in answer to a request. There are
//! four kinds: refreshing a connection's token before it expires, failing a consent that nobody
//! finished, publishing again an event whose first publish failed, and checking a trigger that
//! checks on a schedule (a watched pull request).
//!
//! Each task does one job for one connection or event. It carries everything the worker needs to
//! check that the job is still wanted, and its name is built from what it is for, so scheduling
//! the same job twice creates only one task.
//!
//! Use cases schedule tasks through the [`TaskScheduler`] trait and does not know which service
//! runs them. Each implementation lives in its own crate or module and is chosen when a process
//! starts: Cloud Tasks when deployed (`osiris_cloud_tasks`), local timers in development, and
//! memory in tests. When a task is due, the scheduler sends it as JSON to the worker's
//! `POST /tasks`. A 2xx answer means the task is done; any other answer means the scheduler tries
//! it again later.

pub mod adapters;
pub mod auth_error;
pub mod authenticator;
pub mod consent_task;
pub mod error;
pub mod event_task;
pub mod names;
pub mod payload;
pub mod refresh_reason;
pub mod refresh_task;
pub mod scheduled;
pub mod scheduler;
pub mod task;
#[cfg(any(test, feature = "testing"))]
pub mod testing;
pub mod trigger_task;

pub use auth_error::AuthError;
pub use authenticator::TaskAuthenticator;
pub use consent_task::ConsentTask;
pub use error::TaskError;
pub use event_task::EventTask;
pub use payload::TaskPayload;
pub use refresh_reason::RefreshReason;
pub use refresh_task::RefreshTask;
pub use scheduled::Scheduled;
pub use scheduler::TaskScheduler;
pub use task::Task;
pub use trigger_task::TriggerTask;
