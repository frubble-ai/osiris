//! Errors as error-stack reports: each layer has its own error type, and turns the one below into
//! its own with [`ErrorSwitch`] (database → domain → wire), keeping the chain for the logs.

pub use error_stack::{Report, ResultExt};

/// A result whose error is a report of `E`, carrying every layer's context underneath.
pub type CustomResult<T, E> = Result<T, Report<E>>;

/// How a layer's error becomes the next layer's.
pub trait ErrorSwitch<T> {
    fn switch(&self) -> T;
}

/// Switch a report's error to the next layer's, keeping the report's chain.
pub trait ReportSwitchExt<T, U> {
    fn switch(self) -> CustomResult<T, U>;
}

impl<T, E, U> ReportSwitchExt<T, U> for CustomResult<T, E>
where
    E: ErrorSwitch<U> + std::error::Error + Send + Sync + 'static,
    U: std::error::Error + Send + Sync + 'static,
{
    fn switch(self) -> CustomResult<T, U> {
        self.map_err(|report| {
            let next = report.current_context().switch();
            report.change_context(next)
        })
    }
}
