# Test organization

1. **Tests live beside the code they test.** A module's tests are `tests.rs` or `tests/` in its folder, declared `#[cfg(test)] mod tests;`. A crate's tests that use only its public API are `tests/*.rs`, named for what they exercise.
2. **Mocks and data factories live in `mocks/`** beside what they fake, and follow the [placement rule](./placement-rule.md). Two kinds of file:
   - `mock_<name>.rs`: a behaviour fake, with arranger methods that say what the fake answers (`MockFakeNetwork::answer_refresh`, `MockFakeNetwork::answer_revoke`);
   - `generate_<name>.rs`: a data factory built with `fake`, taking overrides for what a test cares about.
3. **A provider's network is faked in `osiris_providers::<provider>::mocks`** with wiremock: the real provider code and HTTP client run, and only the answers are the test's. Other crates' tests use it through `osiris_providers`'s `testing` feature.
4. **Shared test setup lives in `testing/`** of the crate that owns it: `osiris_server::testing` has the configuration fixture, `setup_test_context`, and the arrangers, observers and request helpers on `TestContext`. `osiris_tasks::testing` and `osiris_events::testing` have the contracts and their data.
5. **No inline mocks in tests.** Every fake is a file in `mocks/` or `testing/`. A test states what the fake answers by calling its arranger.
6. **Every factory uses `fake`.** A test that depends on a value reads it from what it generated. A parser or validator test whose exact input is under test uses a literal.
7. **Cases are parametrized with `rstest`** (`#[rstest]`, `#[case::name(…)]`).
8. **Inputs are randomized.**

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
