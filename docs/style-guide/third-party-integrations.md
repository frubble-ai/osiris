# Third-party integrations

1. **Each vendor gets its own crate**: Google Cloud Tasks is `osiris_cloud_tasks`, Google Pub/Sub is `osiris_pubsub`. Only that crate imports the vendor's library, so replacing the vendor touches one crate.
2. **The use cases depend on a trait of ours**, named for what they need, never for the vendor's API: `TaskScheduler` (`osiris_tasks`) and `Transport` behind `MessageBus` (`osiris_events`). Nothing in `use_cases/` names a vendor, an id, a status or an error of one.
3. **The vendor crate implements our trait.** `CloudTasksScheduler` implements `TaskScheduler`; `PubSubTransport` implements `Transport`. Vendor errors are turned into ours (`TaskError`, `TransportError`) inside the crate.
4. **Adapters that need no vendor live with the trait**: the local and in-memory schedulers in `osiris_tasks/src/adapters/`, the in-memory transport in `osiris_events/src/adapters/`.
5. **One place chooses.** `build_use_case_context` reads the configuration (`config/integrations/`: `OSIRIS_TASKS`, the Pub/Sub variables) and builds the chosen adapter. Nothing else matches on which one it is.
6. **Mirror the domains.** Inside a vendor-shaped crate, split by domain, as the rest of osiris does ([placement rule](./placement-rule.md)).

## Providers

GitHub, Linear and Slack are the providers osiris connects users to. Each is a folder in `osiris_providers/` that implements `Provider` (`osiris_interfaces`) and returns our types; the use cases find one by id through `Providers`. A provider's HTTP is faked at the network in tests ([test organization](./test-organization.md)).

## Contract tests

Every implementation of a trait of ours runs the same contract, so the use cases can rely on what the trait says whichever adapter is behind it:

- `osiris_tasks::testing::task_scheduler_contract` runs against the memory scheduler, the local scheduler, and `CloudTasksScheduler` over Cloud Tasks' REST API faked at the network.
- `osiris_events::testing::transport_contract` runs against the memory transport and `PubSubTransport` over the Pub/Sub emulator.

A new adapter runs the contract through a harness (`SchedulerHarness`, `TransportHarness`) that makes it answer each step.

## Adding an adapter

1. Its crate, implementing the trait, with the vendor's library imported only there.
2. A variant in `config/integrations/` and a branch in `build_use_case_context`.
3. Its contract test.
4. The variables it reads, in [configuration](../configuration.md).
