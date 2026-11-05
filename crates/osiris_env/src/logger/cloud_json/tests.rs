//! Tests of `cloud_json`.

use super::*;
use std::{
    io,
    sync::{Arc, Mutex, PoisonError},
};
use tracing_subscriber::fmt::{MakeWriter, format::JsonFields};

#[derive(Clone, Default)]
struct Buffer(Arc<Mutex<Vec<u8>>>);

impl io::Write for Buffer {
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

impl<'a> MakeWriter<'a> for Buffer {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[test]
fn writes_one_json_line_per_event_with_its_cloud_logging_severity() {
    let buffer = Buffer::default();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(buffer.clone())
        .fmt_fields(JsonFields::new())
        .event_format(CloudJson)
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        let span = tracing::info_span!("request", request_id = "req_1");
        let _entered = span.enter();
        tracing::info!(connected_account_id = "ca_1", "tokens refreshed");
        tracing::warn!("slow");
        tracing::error!(attempt = 3_u64, "failed");
    });
    let text = String::from_utf8(
        buffer
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone(),
    )
    .unwrap_or_default();
    let lines: Vec<Value> = text
        .lines()
        .map(|line| serde_json::from_str(line).unwrap_or(Value::Null))
        .collect();
    assert_eq!(lines.len(), 3, "{text}");
    assert_eq!(
        lines.first().and_then(|line| line.get("severity")),
        Some(&"INFO".into())
    );
    assert_eq!(
        lines.first().and_then(|line| line.get("message")),
        Some(&"tokens refreshed".into())
    );
    assert_eq!(
        lines
            .first()
            .and_then(|line| line.get("connected_account_id")),
        Some(&"ca_1".into())
    );
    assert_eq!(
        lines
            .first()
            .and_then(|line| line.pointer("/span/request_id")),
        Some(&"req_1".into())
    );
    assert!(lines.first().and_then(|line| line.get("time")).is_some());
    assert_eq!(
        lines.get(1).and_then(|line| line.get("severity")),
        Some(&"WARNING".into())
    );
    assert_eq!(
        lines.get(2).and_then(|line| line.get("severity")),
        Some(&"ERROR".into())
    );
    assert_eq!(
        lines.get(2).and_then(|line| line.get("attempt")),
        Some(&3.into())
    );
}
