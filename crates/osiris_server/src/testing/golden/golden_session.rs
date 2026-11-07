//! A scenario's run at one release: each request sent as a client pinned to it sends it, and its
//! answer and the events it published written down, masked, as the transcript's steps.

use super::{masker::Masker, transcript_file::check_transcript};
use crate::testing::{ADMIN_KEY, API_KEY, TestContext, requests::Answer};
use serde_json::{Value, json};

/// The key a step is sent with.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Api,
    Admin,
    Wrong,
    None,
}

impl Key {
    fn secret(self) -> Option<&'static str> {
        match self {
            Self::Api => Some(API_KEY),
            Self::Admin => Some(ADMIN_KEY),
            Self::Wrong => Some("a-wrong-key"),
            Self::None => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Admin => "admin",
            Self::Wrong => "wrong",
            Self::None => "none",
        }
    }
}

/// The answer headers a transcript keeps: the ones a client acts on.
const KEPT_HEADERS: &[&str] = &[
    "osiris-version",
    "osiris-stability",
    "deprecation",
    "sunset",
    "location",
];

pub struct GoldenSession<'a> {
    app: &'a TestContext,
    release: &'static str,
    masker: Masker,
    steps: Vec<Value>,
    events_seen: usize,
}

impl<'a> GoldenSession<'a> {
    pub fn new(app: &'a TestContext, release: &'static str) -> Self {
        Self {
            app,
            release,
            masker: Masker::new(&app.provider.uri()),
            steps: Vec::new(),
            events_seen: 0,
        }
    }

    /// Sends a request pinned to the session's release, with `extra` headers; answers it, and
    /// writes the step down.
    pub async fn send(
        &mut self,
        method: &str,
        path: &str,
        key: Key,
        body: Option<Value>,
        extra: &[(&str, &str)],
    ) -> Answer {
        let mut headers = vec![("osiris-version", self.release)];
        headers.extend_from_slice(extra);
        let answer = self
            .app
            .send(method, path, key.secret(), body.clone(), &headers)
            .await;
        let kept: serde_json::Map<String, Value> = KEPT_HEADERS
            .iter()
            .filter_map(|name| {
                let value = answer.headers.get(*name)?.to_str().ok()?;
                Some(((*name).to_owned(), Value::String(value.to_owned())))
            })
            .collect();
        let events: Vec<Value> = self
            .app
            .bus
            .sent()
            .iter()
            .skip(self.events_seen)
            .map(|sent| serde_json::from_slice(&sent.data).expect("an event's JSON"))
            .collect();
        self.events_seen += events.len();
        let step = json!({
            "request": {
                "method": method,
                "path": path,
                "key": key.label(),
                "headers": extra.iter().map(|(name, value)| json!([name, value])).collect::<Vec<_>>(),
                "body": body,
            },
            "answer": {
                "status": answer.status.as_u16(),
                "headers": kept,
                "body": answer.body,
            },
            "events": events,
        });
        self.steps.push(self.masker.mask(&step));
        answer
    }

    /// Leaves the events published so far out of the transcript: ones its arrangement caused.
    pub fn skip_events(&mut self) {
        self.events_seen = self.app.bus.sent().len();
    }

    /// Records the transcript, or holds what was done against the recorded one.
    pub fn finish(self, scenario: &str) {
        check_transcript(self.release, scenario, &json!({ "steps": self.steps }));
    }
}
