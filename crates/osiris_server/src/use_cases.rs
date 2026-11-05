//! What osiris does. Every operation is a use case in `cases/`, one file each, called through the
//! [`UseCases`] handle: the HTTP routes, the idempotency middleware and the worker call use cases
//! and nothing else, and only use cases reach storage, the providers, the event bus and the task
//! scheduler. The context the use cases run on is built once at boot (`build_use_case_context`)
//! and holds setup only. `lib/` holds the helpers use cases share; a helper takes the parts of the
//! context it needs, never the context, and never calls a use case.

pub mod build_use_case_context;
pub mod cases;
pub mod context;
pub mod lib;
pub mod use_case;

pub use context::UseCaseContext;
pub use lib::shared::domain_errors::UseCaseResult;
pub use use_case::UseCases;
