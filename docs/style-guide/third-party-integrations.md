# Third-party integrations

1. **Each vendor has its own crate**: `osiris_cloud_tasks` for Google Cloud Tasks, `osiris_pubsub`
   for Google Pub/Sub. Only that crate imports the vendor's library.
2. **Use cases depend on a trait**, named for what they need: `TaskScheduler` and `TaskAuthenticator`
   (`osiris_tasks`), and `Transport` behind `MessageBus` (`osiris_events`). Nothing in `use_cases/`
   names a vendor, an id, a status or an error of one.
3. **The vendor crate implements the trait.** `CloudTasksScheduler` implements `TaskScheduler`,
   `OidcAuthenticator` implements `TaskAuthenticator`, `PubSubTransport` implements `Transport`.
   Vendor errors become `TaskError`, `AuthError` and `TransportError` inside the crate.
4. **Implementations that need no vendor** live with the trait: the local and in-memory schedulers
   and the bearer-token authenticator in `osiris_tasks/src/adapters/`, the in-memory transport in
   `osiris_events/src/adapters/`.
5. **`build_use_case_context` builds the chosen implementation** from `config/integrations/`
   (`OSIRIS_TASKS`, the Pub/Sub variables); the worker's authenticator is built beside the worker.

## Providers

The providers users connect to are each a folder in `osiris_providers/` that implements
`Provider` (`osiris_interfaces`) and registers itself (`registration.rs`); use cases find one by
id, or by type, through `Providers`. Each provider's page is in [providers](../providers.md). A provider's HTTP is faked at the network in tests
([test organization](./test-organization.md)).

## Contract tests

Every implementation of a trait runs the same contract:

- `osiris_tasks::testing::task_scheduler_contract`: the memory scheduler, the local scheduler, and
  `CloudTasksScheduler` over Cloud Tasks' REST API faked at the network.
- `osiris_events::testing::transport_contract`: the memory transport and `PubSubTransport` over the
  Pub/Sub emulator.

A new implementation runs the contract through a harness (`SchedulerHarness`, `TransportHarness`)
that makes it answer each step.

## Adding an implementation

1. Its crate, implementing the trait, with the vendor's library imported only there.
2. A variant in `config/integrations/` and a branch in `build_use_case_context`.
3. Its contract test.
4. The variables it reads, in [configuration](../configuration.md).
