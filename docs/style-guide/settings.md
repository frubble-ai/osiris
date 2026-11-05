# Settings

A value someone might tune per environment (a batch size, a margin, a delay, a timeout, a
retention period, a pool size, a URL) is a setting: a field of its section in
`config/settings/`, with its default in that section's `Default` and its line in
`config.example.toml`. It is passed into the code that uses it (for example, storage takes the
idempotency retention as an argument).

Only facts stay constants: a provider's documented page size, header names, the stored format's
version. Never a literal inside SQL: bind it (`make_interval(secs => $1)`).

Secrets and infrastructure are environment variables, read once, typed `Secret` when secret
([configuration](../configuration.md)).
