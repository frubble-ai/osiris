# Errors

- **One error type per layer**: `StorageError` → `DomainError` → the wire's `ApiError`. A layer turns the one below into its own with `ErrorSwitch` (`.switch()`), keeping the report.
- **Reports, not strings**: results are `CustomResult<T, E>` (error-stack); use cases return `UseCaseResult<T>`. A cause stays in the report's chain for the logs; `change_context` names what failed at this layer.
- **The wire** answers `{ "error": { "code", "type", "message" } }`. `code` (`OS_00`…`OS_99`) is stable; a client branches on it. A message never carries a provider's text or a cause; the log does.
- **Providers** fail as `Unauthorized` (the token is no longer accepted), `Refused` (a 4xx or their own refusal) or `Unavailable` (an outage or a timeout: 408, 429, 5xx, a network error; try again). The use cases decide what each means: a refusal ends the connection, an outage keeps it and retries.
- **What a client may read back** (`last_error`) is a fixed message, never the provider's text and never anything a caller sent.
- **No panics** outside tests: no `unwrap`, `expect`, indexing or `as` casts (the lints refuse them). Tests allow what they need with a stated reason.
