-- migrate:up
CREATE TABLE auth.api_keys (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    key_prefix TEXT NOT NULL UNIQUE,
    secret_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ
);

COMMENT ON TABLE auth.api_keys IS
'One Runtime API keys used by MCP/API clients. Only hashed secret material is stored.';

COMMENT ON COLUMN auth.api_keys.key_prefix IS
'Public lookup prefix embedded in the bearer token. Used to find the key record without storing plaintext secrets.';

COMMENT ON COLUMN auth.api_keys.secret_hash IS
'SHA-256 hash of the secret portion of the API key. Plaintext key material is never persisted.';

CREATE INDEX auth_api_keys_user_idx
    ON auth.api_keys (user_id);

CREATE INDEX auth_api_keys_org_idx
    ON auth.api_keys (org_id);

GRANT SELECT, INSERT, UPDATE, DELETE ON auth.api_keys TO application_user;
GRANT SELECT ON auth.api_keys TO application_readonly;

-- migrate:down
DROP TABLE IF EXISTS auth.api_keys;
