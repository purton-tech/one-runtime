-- migrate:up
ALTER TABLE public.integration_connections
    DROP CONSTRAINT integration_connections_auth_fields_chk;

ALTER TABLE public.integration_connections
    ADD COLUMN api_key TEXT,
    ADD COLUMN end_user_id TEXT,
    ADD COLUMN end_user_name TEXT,
    ADD COLUMN end_user_email TEXT;

ALTER TABLE public.integration_connections
    DROP COLUMN api_key_secret_ref;

ALTER TABLE public.integration_connections
    ADD CONSTRAINT integration_connections_auth_fields_chk CHECK (
        (
            auth_type = 'api_key'
            AND api_key IS NOT NULL
            AND oauth_access_token_secret_ref IS NULL
            AND oauth_refresh_token_secret_ref IS NULL
        )
        OR
        (
            auth_type = 'oauth2'
            AND api_key IS NULL
            AND oauth_access_token_secret_ref IS NOT NULL
        )
    );

COMMENT ON TABLE public.integration_connections IS
'Per-org configured credentials for an integration. Usually private to a user; optionally shared with the org. API keys are currently stored directly in Postgres; OAuth2 tokens still use external references.';

COMMENT ON COLUMN public.integration_connections.api_key IS
'API key credential for auth_type=api_key. Stored directly in Postgres for now.';

COMMENT ON COLUMN public.integration_connections.end_user_id IS
'Customer-defined end-user identifier associated with this connection.';

COMMENT ON COLUMN public.integration_connections.end_user_name IS
'Optional customer-defined display name for the end user associated with this connection.';

COMMENT ON COLUMN public.integration_connections.end_user_email IS
'Optional customer-defined email for the end user associated with this connection.';

CREATE TABLE public.hosted_connection_sessions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    created_by_api_key_id UUID NOT NULL REFERENCES auth.api_keys(id) ON DELETE CASCADE,
    integration_id UUID NOT NULL REFERENCES public.integrations(id) ON DELETE CASCADE,
    integration_slug TEXT NOT NULL,
    end_user_id TEXT NOT NULL,
    end_user_name TEXT,
    end_user_email TEXT,
    suggested_connection_name TEXT,
    auth_type integration_auth_type NOT NULL,
    token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT hosted_connection_sessions_token_nonempty_chk CHECK (length(trim(token)) > 0),
    CONSTRAINT hosted_connection_sessions_end_user_id_nonempty_chk CHECK (length(trim(end_user_id)) > 0)
);

COMMENT ON TABLE public.hosted_connection_sessions IS
'Short-lived server-created sessions used by the hosted popup to create exactly one integration connection.';

CREATE INDEX hosted_connection_sessions_expires_idx
    ON public.hosted_connection_sessions (expires_at);

CREATE INDEX hosted_connection_sessions_org_idx
    ON public.hosted_connection_sessions (org_id, created_at DESC);

GRANT SELECT, INSERT, UPDATE, DELETE ON public.hosted_connection_sessions TO application_user;
GRANT SELECT ON public.hosted_connection_sessions TO application_readonly;

-- migrate:down
DROP TABLE IF EXISTS public.hosted_connection_sessions;

ALTER TABLE public.integration_connections
    DROP CONSTRAINT integration_connections_auth_fields_chk;

ALTER TABLE public.integration_connections
    ADD COLUMN api_key_secret_ref TEXT;

ALTER TABLE public.integration_connections
    DROP COLUMN end_user_email,
    DROP COLUMN end_user_name,
    DROP COLUMN end_user_id,
    DROP COLUMN api_key;

ALTER TABLE public.integration_connections
    ADD CONSTRAINT integration_connections_auth_fields_chk CHECK (
        (auth_type = 'api_key'
            AND api_key_secret_ref IS NOT NULL
            AND oauth_access_token_secret_ref IS NULL
            AND oauth_refresh_token_secret_ref IS NULL
        )
        OR
        (auth_type = 'oauth2'
            AND oauth_access_token_secret_ref IS NOT NULL
        )
    );

COMMENT ON TABLE public.integration_connections IS
'Per-org configured credentials for an integration. Usually private to a user; optionally shared with the org. Secrets are stored externally and referenced here.';

COMMENT ON COLUMN public.integration_connections.api_key_secret_ref IS
'Secret store reference for API key credentials (auth_type=api_key).';
