# Errors

- **One error type per layer**: `StorageError` → `DomainError` → the wire's `ApiError`. A layer
  turns the one below into its own with `ErrorSwitch` (`.switch()`), keeping the report.
- **Reports, not strings**: results are `CustomResult<T, E>` (error-stack). A cause stays in the
  report's chain for the logs; `change_context` names what failed at this layer.
- **The wire** answers `{ "error": { "code", "type", "message" } }`. `code` (`OS_00`…`OS_99`) is
  stable; a client branches on it. A 5xx's message never carries its cause; the log does.
- **Providers** fail as `Unauthorized` (the token is no longer accepted), `Refused` (a 4xx or
  their own refusal) or `Unavailable` (try again). The core decides what each means.
- **No panics** outside tests: no `unwrap`, `expect`, indexing or `as` casts (the lints refuse
  them).
