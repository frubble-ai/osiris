-- A trigger: a connection's subscription to something happening at the provider (a webhook's
-- events, a watched item's changes). The provider sends deliveries to
-- osiris, signed with the trigger's own secret (encrypted, never answered), and osiris publishes
-- each one as a `trigger.fired` event. `subscription_id` is the provider's id for what osiris
-- created there (such as a webhook); `checkpoint` is what a trigger that also checks on a
-- schedule saw last.
CREATE TABLE trigger_status (key text PRIMARY KEY);
INSERT INTO trigger_status (key) VALUES ('active'), ('failed');

CREATE TABLE trigger (
    id text PRIMARY KEY,
    connected_account_id text NOT NULL REFERENCES connected_account (id) ON DELETE CASCADE,
    -- The connection's provider and the trigger's kind, `<provider>.<kind>`.
    provider text NOT NULL REFERENCES provider (key),
    kind text NOT NULL,
    config jsonb NOT NULL,
    secret bytea NOT NULL,
    status text NOT NULL REFERENCES trigger_status (key),
    subscription_id text,
    checkpoint jsonb,
    -- Why the trigger last failed (the provider refused a check, the subscription is gone), kept
    -- for the client; null while it works.
    last_error text,
    -- The pending scheduled check's name and when it is due, for a kind that checks on a schedule.
    poll_task text,
    poll_task_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX trigger_connected_account_idx ON trigger (connected_account_id);
CREATE INDEX trigger_created_idx ON trigger (created_at, id);
