-- A provider's app may carry settings of its own, which the provider validates: `settings` it may
-- answer (the app's id at the provider), and `secret_settings`, encrypted like the client secret
-- and never answered (the key the app signs with).
ALTER TABLE auth_config
    ADD COLUMN settings jsonb NOT NULL DEFAULT '{}'::jsonb,
    ADD COLUMN secret_settings bytea;

-- A connection acts either as the user who consented (`oauth`, its tokens kept here) or as an
-- app's installation (`app_installation`, its tokens minted on demand from the app's credentials
-- and never stored). `installation_id` is the provider's id of that installation.
CREATE TABLE credential_kind (key text PRIMARY KEY);
INSERT INTO credential_kind (key) VALUES ('oauth'), ('app_installation');

ALTER TABLE connected_account
    ADD COLUMN credential_kind text NOT NULL DEFAULT 'oauth' REFERENCES credential_kind (key),
    ADD COLUMN installation_id text,
    ADD CONSTRAINT connected_account_installation CHECK (
        (credential_kind = 'app_installation') = (installation_id IS NOT NULL)
    );

-- Never half connected: an active OAuth connection holds its tokens, and a refreshable one its
-- scheduled refresh; an active installation connection holds no tokens at all.
ALTER TABLE connected_account DROP CONSTRAINT connected_account_complete;
ALTER TABLE connected_account ADD CONSTRAINT connected_account_complete CHECK (
    status <> 'active'
    OR (
        credential_kind = 'oauth'
        AND access_token IS NOT NULL
        AND (refresh_token IS NULL OR refresh_task IS NOT NULL)
    )
    OR (credential_kind = 'app_installation' AND access_token IS NULL)
);
