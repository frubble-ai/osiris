//! `golden!`: a scenario replayed at every release, one test each, named after its release. Each
//! runs the scenario on a database of its own and holds it to that release's recorded transcript,
//! so a release without the scenario's transcript fails until it is recorded.

/// The scenario `$run` at every release (`for_every_release!`), its transcript kept under
/// `$scenario`.
macro_rules! golden {
    ($scenario:literal, $run:path) => {
        for_every_release!(golden_at!($scenario, $run,));
    };
}

/// The scenario `$run` at each listed release.
macro_rules! golden_at {
    ($scenario:literal, $run:path, [$($module:ident $release:literal),* $(,)?]) => {
        $(
            #[sqlx::test(migrator = "crate::db::MIGRATOR")]
            async fn $module(pool: sqlx::PgPool) {
                let app = crate::testing::setup_test_context(pool).await;
                let mut session = crate::testing::golden::GoldenSession::new(&app, $release);
                $run(&app, &mut session).await;
                session.finish($scenario);
            }
        )*
    };
}
