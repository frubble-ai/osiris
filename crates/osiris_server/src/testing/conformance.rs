//! Every answer and event a test sees is checked against the spec of the release it was written
//! in, as Schemathesis checks a live API: its status must be one the operation documents, and its
//! body must match that answer's schema. A test can't pass on an answer the spec doesn't promise.

pub mod assert_answer_conforms;
pub mod assert_event_conforms;
pub mod schema_validator;
pub mod template_of;

pub use assert_answer_conforms::assert_answer_conforms;
pub use assert_event_conforms::assert_event_conforms;
