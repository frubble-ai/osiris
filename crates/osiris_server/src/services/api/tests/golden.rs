//! Each release's golden transcripts, replayed: every scenario runs at every release osiris
//! serves, on a database of its own, and must still do what was recorded for that release
//! (`testing::golden`). A release added to `RELEASES` gets a line in `golden_at!` below; a test
//! fails until every release has every scenario's transcript.

mod actions;
mod auth_configs;
mod connecting;
mod events;
mod idempotency;
mod refusals;

use crate::testing::golden::transcript_file::transcript_path;
use osiris_api_models::version::RELEASES;

/// Every scenario, by the name its transcript is written under.
const SCENARIOS: &[&str] = &[
    "actions",
    "auth_configs",
    "connecting",
    "events",
    "idempotency",
    "refusals",
];

/// One test per scenario at `release`, in a module named after it.
macro_rules! golden_at {
    ($module:ident, $release:literal) => {
        mod $module {
            use crate::testing::{golden::GoldenSession, setup_test_context};
            use sqlx::PgPool;

            const RELEASE: &str = $release;

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn actions(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                super::actions::run(&app, &mut session).await;
                session.finish("actions");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn auth_configs(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                super::auth_configs::run(&mut session).await;
                session.finish("auth_configs");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn connecting(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                super::connecting::run(&app, &mut session).await;
                session.finish("connecting");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn events(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                super::events::run(&app, &mut session).await;
                session.finish("events");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn idempotency(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                super::idempotency::run(&mut session).await;
                session.finish("idempotency");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn refusals(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                super::refusals::run(&mut session).await;
                session.finish("refusals");
            }
        }
    };
}

golden_at!(anubis, "2026-11-04.anubis");

#[test]
fn keeps_every_scenario_s_transcript_for_every_release() {
    for release in RELEASES {
        for scenario in SCENARIOS {
            let path = transcript_path(release.name, scenario);
            assert!(
                path.exists() || std::env::var("OSIRIS_GOLDEN").as_deref() == Ok("record"),
                "{release:?} has no {scenario} transcript at {}: add `golden_at!` for it and \
                 record it with `OSIRIS_GOLDEN=record just test golden`",
                path.display()
            );
        }
    }
}
