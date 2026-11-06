//! `osiris listen` against the emulator from docker-compose (`PUBSUB_EMULATOR_HOST`, with the
//! `osiris-events` topic `just up` creates): an event published through the transport is `POST`ed
//! to the endpoint as Pub/Sub would push it, and the subscription is gone afterwards. Locally,
//! without the emulator the test is skipped with a note; in CI (`CI` set) it must run.

#![allow(
    clippy::print_stderr,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a skip is reported on stderr; a failure here is the test failing"
)]

use base64::{Engine, engine::general_purpose::STANDARD};
use fake::Fake;
use google_cloud_pubsub::client::SubscriptionAdmin;
use osiris_events::{Message, MessageBus, testing::generate_event_message::generate_event_message};
use osiris_pubsub::{
    PubSubConfig, PubSubTransport,
    listen::{Listen, listen},
    mocks::mock_push_endpoint::MockPushEndpoint,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::{mpsc, oneshot};
use wiremock::MockServer;

const LONG_ENOUGH: Duration = Duration::from_secs(20);

type Forwards = mpsc::UnboundedReceiver<(String, Result<u16, String>)>;

struct Listening {
    stop: oneshot::Sender<()>,
    forwards: Forwards,
    task: tokio::task::JoinHandle<anyhow::Result<()>>,
}

fn start_listening(config: &PubSubConfig, forward_to: String, suffix: String) -> Listening {
    let (stop, stopped) = oneshot::channel::<()>();
    let (forwarded, forwards) = mpsc::unbounded_channel();
    let config = config.clone();
    let task = tokio::spawn(async move {
        listen(
            Listen {
                pubsub: &config,
                ack_deadline_seconds: 10,
                subscription_suffix: &suffix,
                forward_to: &forward_to,
            },
            async move {
                let _ = stopped.await;
            },
            move |each| {
                let _ = forwarded.send((each.event_type, each.answer));
            },
        )
        .await
    });
    Listening {
        stop,
        forwards,
        task,
    }
}

#[tokio::test]
async fn forwards_a_published_event_as_a_push_and_removes_its_subscription() {
    let Some(host) = std::env::var("PUBSUB_EMULATOR_HOST")
        .ok()
        .filter(|host| !host.is_empty())
    else {
        assert!(
            std::env::var_os("CI").is_none(),
            "PUBSUB_EMULATOR_HOST is not set in CI"
        );
        eprintln!("skipped: PUBSUB_EMULATOR_HOST is not set");
        return;
    };
    let config = PubSubConfig {
        project: "osiris-local".into(),
        events_topic: "osiris-events".into(),
        emulator_host: Some(host.clone()),
    };
    let endpoint = MockServer::start().await;
    let push_endpoint = MockPushEndpoint::on(&endpoint);
    push_endpoint.answer_push(204).await;
    let suffix = format!(
        "test-{}",
        16.fake::<String>()
            .replace(|character: char| !character.is_ascii_alphanumeric(), "x")
    );
    let subscription = format!(
        "projects/{}/subscriptions/osiris-cli-{suffix}",
        config.project
    );
    let admin = SubscriptionAdmin::builder()
        .with_endpoint(format!("http://{host}"))
        .with_credentials(google_cloud_auth::credentials::anonymous::Builder::new().build())
        .build()
        .await
        .unwrap();
    let mut listening = start_listening(&config, push_endpoint.url(), suffix);

    // An event published before the subscription exists would never reach it.
    tokio::time::timeout(LONG_ENOUGH, async {
        while admin
            .get_subscription()
            .set_subscription(&subscription)
            .send()
            .await
            .is_err()
        {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the subscription is created");
    let bus = MessageBus::new(Arc::new(PubSubTransport::connect(&config).await.unwrap()));
    let event = generate_event_message();
    bus.send(Message::Event(event.clone())).await.unwrap();

    // The topic is shared, so other events may be forwarded too: wait for the one published here.
    let ours = tokio::time::timeout(LONG_ENOUGH, async {
        loop {
            let (_, answer) = listening
                .forwards
                .recv()
                .await
                .expect("an event is forwarded");
            assert_eq!(answer, Ok(204));
            let pushes = endpoint.received_requests().await.unwrap();
            let found = pushes.iter().find_map(|request| {
                let push: serde_json::Value = serde_json::from_slice(&request.body).ok()?;
                let data = STANDARD.decode(push["message"]["data"].as_str()?).ok()?;
                String::from_utf8_lossy(&data)
                    .contains(&event.event_id)
                    .then_some(push)
            });
            if let Some(push) = found {
                return push;
            }
        }
    })
    .await
    .expect("the published event is forwarded");
    assert_eq!(ours["subscription"], subscription);
    assert_eq!(
        ours["message"]["attributes"]["type"],
        event.event_type.as_str()
    );

    listening.stop.send(()).unwrap();
    tokio::time::timeout(LONG_ENOUGH, listening.task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(
        admin
            .get_subscription()
            .set_subscription(&subscription)
            .send()
            .await
            .is_err(),
        "the subscription is removed"
    );
}
