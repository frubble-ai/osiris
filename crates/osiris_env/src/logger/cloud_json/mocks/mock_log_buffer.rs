//! A log sink that keeps what is written to it.

use std::{
    io,
    sync::{Arc, Mutex, PoisonError},
};
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone, Default)]
pub struct MockLogBuffer(Arc<Mutex<Vec<u8>>>);

impl MockLogBuffer {
    /// Everything written so far.
    pub fn text(&self) -> String {
        String::from_utf8(
            self.0
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone(),
        )
        .unwrap_or_default()
    }
}

impl io::Write for MockLogBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for MockLogBuffer {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}
