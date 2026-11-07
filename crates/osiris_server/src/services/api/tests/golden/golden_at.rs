//! `golden_at!`: one test per scenario at a release, in a module named after it. Each runs the
//! scenario on a database of its own and holds it to that release's recorded transcript.

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
                crate::services::api::tests::golden::actions::run(&app, &mut session).await;
                session.finish("actions");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn auth_configs(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                crate::services::api::tests::golden::auth_configs::run(&mut session).await;
                session.finish("auth_configs");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn connecting(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                crate::services::api::tests::golden::connecting::run(&app, &mut session).await;
                session.finish("connecting");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn events(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                crate::services::api::tests::golden::events::run(&app, &mut session).await;
                session.finish("events");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn idempotency(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                crate::services::api::tests::golden::idempotency::run(&mut session).await;
                session.finish("idempotency");
            }

            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn refusals(pool: PgPool) {
                let app = setup_test_context(pool).await;
                let mut session = GoldenSession::new(&app, RELEASE);
                crate::services::api::tests::golden::refusals::run(&mut session).await;
                session.finish("refusals");
            }
        }
    };
}
