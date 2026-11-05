# Test organization

1. **Tests live beside the code they test.** A module's unit and behaviour tests are `tests.rs` or `tests/` in its folder, declared `#[cfg(test)] mod tests;`. A crate's integration tests that need only its public API are `tests/*.rs`, named for the thing they exercise.
2. **Mocks and data factories live in `mocks/`** beside what they fake, and follow the [placement rule](./placement-rule.md). Two kinds of file:
   - `mock_<name>.rs`: a behaviour replacement, with arranger methods that say what the fake answers (`MockGithubNetwork::answer_refresh`, `MockSlackNetwork::answer_channels_page`);
   - `generate_<name>.rs`: a data factory built with `fake`, taking overrides for what a test cares about.
3. **A provider's network is faked once.** `osiris_providers::<provider>::mocks` fakes the provider's HTTP with wiremock, so the real provider code and the real HTTP client run and only the answers are the test's. Other crates' tests use it through `osiris_providers`'s `testing` feature.
4. **Shared test setup lives in `testing/`** of the crate that owns it: `osiris_server::testing` has the configuration fixture, `setup_test_context`, and the arrangers, observers and request helpers on `TestContext`. `osiris_tasks::testing` and `osiris_events::testing` have the contracts and their data.
5. **No inline mocks in tests.** Every fake is a file in `mocks/` or `testing/` that the test uses. A test body states what the fake answers by calling its arranger.
6. **Every factory uses `fake`.** No hardcoded test data; a test that depends on a value reads it from what it generated. Exception: a parser or validator test whose exact input is the thing under test uses a literal.
7. **Parametrize with `rstest`** (`#[rstest]`, `#[case::name(…)]`) instead of repeating a test or looping in its body.
8. **Randomize inputs.** A test that passes only for one value it was written with proves little.

## Layout

```
osiris_server/src/
├── testing.rs  testing/              # TestContext and its arrangers, observers, requests
├── use_cases/cases/tests.rs  tests/  # what the use cases do, by domain
├── services/api/tests.rs  tests/     # the API's contract through its router, by domain
├── services/worker/tests.rs          # who may send a task; /health while draining
osiris_providers/src/<provider>/
├── mocks.rs  mocks/                  # the provider's faked network and factories
└── tests.rs                          # the provider against it
osiris_tasks/src/testing.rs           # the TaskScheduler contract
osiris_events/src/testing.rs          # the Transport contract
```
