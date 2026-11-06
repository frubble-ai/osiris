//! Tests of `cloud_json`.

use super::mocks::mock_log_buffer::MockLogBuffer;
use super::*;
use tracing_subscriber::fmt::format::JsonFields;

#[test]
fn writes_one_json_line_per_event_with_its_cloud_logging_severity() {
    let buffer = MockLogBuffer::default();
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
    let text = buffer.text();
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
