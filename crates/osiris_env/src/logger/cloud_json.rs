//! One JSON object per line, in the shape Cloud Logging reads: `severity` (`DEBUG`, `INFO`,
//! `WARNING`, `ERROR`), `time` (RFC 3339), `message`, `target`, the event's own fields, and the
//! fields of the spans it happened in under `span` (for example a request's `request_id`).

use serde_json::{Map, Value};
use std::fmt;
use tracing::{Event, Level, Subscriber, field::Field};
use tracing_subscriber::{
    field::Visit,
    fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields, format::Writer},
    registry::LookupSpan,
};

pub struct CloudJson;

impl<S, N> FormatEvent<S, N> for CloudJson
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        context: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut fields = Map::new();
        event.record(&mut Fields(&mut fields));
        let mut line = Map::new();
        line.insert(
            "severity".into(),
            severity(*event.metadata().level()).into(),
        );
        if let Ok(time) =
            time::OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339)
        {
            line.insert("time".into(), time.into());
        }
        line.insert(
            "message".into(),
            fields.remove("message").unwrap_or_else(|| "".into()),
        );
        line.insert("target".into(), event.metadata().target().into());
        if let Some(scope) = context.event_scope() {
            let mut spans = Map::new();
            for span in scope.from_root() {
                if let Some(formatted) = span.extensions().get::<FormattedFields<N>>()
                    && let Ok(Value::Object(span_fields)) =
                        serde_json::from_str::<Value>(formatted.as_str())
                {
                    spans.extend(span_fields);
                }
            }
            if !spans.is_empty() {
                line.insert("span".into(), Value::Object(spans));
            }
        }
        line.extend(fields);
        writeln!(writer, "{}", Value::Object(line))
    }
}

/// A tracing level as a Cloud Logging severity.
pub fn severity(level: Level) -> &'static str {
    match level {
        Level::ERROR => "ERROR",
        Level::WARN => "WARNING",
        Level::INFO => "INFO",
        Level::DEBUG | Level::TRACE => "DEBUG",
    }
}

/// Collects an event's fields as JSON values.
struct Fields<'a>(&'a mut Map<String, Value>);

impl Visit for Fields<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.0
            .insert(field.name().into(), format!("{value:?}").into());
    }
}

#[cfg(test)]
mod tests;
