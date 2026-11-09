DELETE FROM connected_account WHERE credential_kind = 'app_installation';
ALTER TABLE connected_account DROP CONSTRAINT connected_account_complete;
ALTER TABLE connected_account ADD CONSTRAINT connected_account_complete CHECK (
    status <> 'active'
    OR (access_token IS NOT NULL AND (refresh_token IS NULL OR refresh_task IS NOT NULL))
);
ALTER TABLE connected_account
    DROP CONSTRAINT connected_account_installation,
    DROP COLUMN installation_id,
    DROP COLUMN credential_kind;
DROP TABLE credential_kind;
ALTER TABLE auth_config DROP COLUMN secret_settings, DROP COLUMN settings;
