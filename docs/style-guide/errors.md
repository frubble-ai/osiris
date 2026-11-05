# Errors

- **Two error types**: `DomainError` (what use cases and the database's operations fail with) and the wire's `ApiError`, which is built from it with `From<Report<DomainError>>`, keeping the report. A database failure is a `DomainError::Internal` report whose cause is in the logs.
- **Reports, not strings**: results are `CustomResult<T, E>` (error-stack); use cases return `UseCaseResult<T>`. A cause stays in the report's chain for the logs; `change_context` names what failed.
- **The wire** answers `{ "error": { "code", "type", "message" } }`. `code` (`OS_00`…`OS_99`) is stable; a client branches on it. A message never carries a provider's text or a cause; the log does.
- **Providers** fail as `Unauthorized` (the token is no longer accepted), `Refused` (a 4xx or their own refusal) or `Unavailable` (an outage or a timeout: 408, 429, 5xx, a network error; try again). A refusal ends the connection; an outage keeps it and retries.
- **`last_error`**, which a client reads, is a fixed message: never the provider's text and never anything a caller sent.
- **No panics** outside tests: no `unwrap`, `expect`, indexing or `as` casts (the lints refuse them). Tests allow what they need, with a stated reason.
