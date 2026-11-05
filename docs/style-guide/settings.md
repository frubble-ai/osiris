# Settings and configuration

Configuration reaches the code the way the app's does: read from the environment once at boot into a typed `Config`, validated, and handed to the use cases as `ctx.config`.

- **Secrets and infrastructure are environment variables** (`config/app_config.rs`, `config/integrations/`, `config/database_config.rs`), read once, typed `Secret` when secret. A required one that is missing or malformed stops the process at startup, naming it. No file is read, and nothing depends on which environment osiris runs in.
- **A value someone might tune** (a batch size, a margin, a delay, a timeout, a retention period, a pool size) is a setting: a field of its section in `config/settings/`, with its default in that section's `Default`, its line in `config.example.toml`, and a check in `validate` when it must be positive. Any setting can be changed with `OSIRIS__<SECTION>__<KEY>`. A test fails if `config.example.toml` and the defaults disagree, and an unknown key is refused.
- **Settings reach a function as arguments.** A use case reads `ctx.config.settings.<section>` and passes the section (or one value) to `lib/` and to storage, which never read configuration themselves.
- **Only facts stay constants**: a provider's documented page size, header names, the stored format's version. Never a literal inside SQL: bind it (`make_interval(secs => $1)`).
- **The vendor to use is configuration** (`OSIRIS_TASKS`, the Pub/Sub variables), chosen in `build_use_case_context`; no code branches on the environment.

See [configuration](../configuration.md) for every variable.
