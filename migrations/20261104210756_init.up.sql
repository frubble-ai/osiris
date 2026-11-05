-- The providers osiris implements, and where a connection stands: rows of reference tables, not
-- enums. A provider is a row here plus its own module in crates/osiris_providers/src/; nothing else names one.
CREATE TABLE provider (
    key text PRIMARY KEY
);
INSERT INTO provider (key) VALUES ('github'), ('linear'), ('slack');

CREATE TABLE connection_status (
    key text PRIMARY KEY
);
-- pending: the provider's consent is open. active: tokens held and, when they can be refreshed,
-- their refresh scheduled. disabled: not usable because no refresh is scheduled. At connect no
-- tokens are kept and the user must connect again; after a refresh whose next task could not be
-- scheduled, the new tokens are kept and the refresh is scheduled again by the task's retry or the
-- next action. expired: the provider refused a refresh for good (revoked, uninstalled, refresh
-- token too old), so the user must reconnect. failed: the consent was refused, its code could not
-- be exchanged, or it was never finished.
INSERT INTO connection_status (key) VALUES ('pending'), ('active'), ('disabled'), ('expired'), ('failed');

-- Each tenant's data key, wrapped by the master key (OSIRIS_ENCRYPTION_KEY). A tenant is a
-- connection's user, or osiris itself for the auth configs' secrets. A key is made on first use.
CREATE TABLE data_key (
    tenant text PRIMARY KEY,
    wrapped_key bytea NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

-- A client's app at a provider: its OAuth client and the scopes it asks for. The secret is
-- encrypted with osiris's own data key; osiris never returns it.
CREATE TABLE auth_config (
    id text PRIMARY KEY,
    provider text NOT NULL REFERENCES provider (key),
    client_id text NOT NULL,
    client_secret bytea NOT NULL,
    scopes text[] NOT NULL DEFAULT '{}',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX auth_config_created_idx ON auth_config (created_at, id);

-- One user's connection to a provider through an auth config. `user_id` and `alias` are the
-- client's: who it is for, and the client's own mark for the attempt that made it.
CREATE TABLE connected_account (
    id text PRIMARY KEY,
    auth_config_id text NOT NULL REFERENCES auth_config (id) ON DELETE CASCADE,
    -- The auth config's provider, kept on the connection so reading one needs no join.
    provider text NOT NULL REFERENCES provider (key),
    user_id text NOT NULL,
    alias text,
    status text NOT NULL DEFAULT 'pending' REFERENCES connection_status (key),
    -- Where the browser goes after the provider's consent, whatever the outcome.
    callback_url text NOT NULL,
    -- The OAuth `state` sent to the provider while the consent is open, and until when it may come
    -- back; both cleared once used. The PKCE verifier, when the provider uses PKCE, is encrypted.
    oauth_state text UNIQUE,
    oauth_state_expires_at timestamptz,
    pkce_verifier bytea,
    -- Encrypted, as the client secret is. Null until the consent finishes.
    access_token bytea,
    refresh_token bytea,
    access_token_expires_at timestamptz,
    -- When the refresh token was last issued, so a keep-alive refresh runs before it ages out.
    refresh_token_issued_at timestamptz,
    scope text,
    -- Why the last refresh or the consent failed, kept for the client and the logs; null once a
    -- refresh succeeds. `refresh_failures` counts the failures since.
    last_refresh_error text,
    refresh_failures integer NOT NULL DEFAULT 0,
    -- Moves on with every token write; a refresh task names the generation it was made for and
    -- is stale on any other.
    token_generation bigint NOT NULL DEFAULT 0,
    -- The pending refresh task's name and when it is due, written in the same statement as the
    -- tokens it refreshes; null when nothing is scheduled.
    refresh_task text,
    refresh_task_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    -- Never half connected: an active connection holds its tokens, and a refreshable one its
    -- scheduled refresh.
    CONSTRAINT connected_account_complete CHECK (
        status <> 'active'
        OR (access_token IS NOT NULL AND (refresh_token IS NULL OR refresh_task IS NOT NULL))
    )
);

CREATE INDEX connected_account_user_idx ON connected_account (user_id, provider);
CREATE INDEX connected_account_created_idx ON connected_account (created_at, id);
CREATE INDEX connected_account_auth_config_idx ON connected_account (auth_config_id);
CREATE INDEX connected_account_alias_idx ON connected_account (alias) WHERE alias IS NOT NULL;

-- Idempotency keys (`Idempotency-Key` on a POST or DELETE): the first answer is kept for
-- `[idempotency] retention` and replayed to a retry with the same key; the same key with another
-- request is refused. A key past its retention reads as absent, and expired keys are deleted as
-- new ones are written.
CREATE TABLE idempotency_key (
    key text PRIMARY KEY,
    request_hash text NOT NULL,
    status integer,
    body jsonb,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX idempotency_key_created_idx ON idempotency_key (created_at);

-- How each task run ended (refreshed, rescheduled, skipped_stale, failed, consent_expired,
-- published), one row
-- per run; rows past `[tasks] outcome_retention` are deleted as new ones are written.
CREATE TABLE task_outcome (
    id bigserial PRIMARY KEY,
    task_name text NOT NULL,
    kind text NOT NULL,
    connected_account_id text NOT NULL,
    outcome text NOT NULL,
    detail text,
    recorded_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX task_outcome_connected_account_idx ON task_outcome (connected_account_id, recorded_at);
CREATE INDEX task_outcome_recorded_idx ON task_outcome (recorded_at);
