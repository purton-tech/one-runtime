-- migrate:up
CREATE TABLE public.oauth_clients (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (org_id, provider, client_id)
);

COMMENT ON TABLE public.oauth_clients IS
'Org-scoped OAuth2 client credentials. Client secrets are currently stored directly in Postgres.';

COMMENT ON COLUMN public.oauth_clients.provider IS
'Provider identifier for the OAuth2 client, such as google or airtable.';

COMMENT ON COLUMN public.oauth_clients.client_id IS
'OAuth2 client identifier issued by the provider.';

COMMENT ON COLUMN public.oauth_clients.client_secret IS
'OAuth2 client secret issued by the provider.';

CREATE INDEX oauth_clients_org_idx
    ON public.oauth_clients (org_id, created_at DESC);

CREATE INDEX oauth_clients_org_provider_idx
    ON public.oauth_clients (org_id, provider);

GRANT SELECT, INSERT, UPDATE, DELETE ON public.oauth_clients TO application_user;
GRANT SELECT ON public.oauth_clients TO application_readonly;

ALTER TABLE public.oauth_clients ENABLE ROW LEVEL SECURITY;

CREATE POLICY oauth_clients_select
ON public.oauth_clients
FOR SELECT
USING (
    org.is_org_member(org_id)
);

CREATE POLICY oauth_clients_insert
ON public.oauth_clients
FOR INSERT
WITH CHECK (
    org.is_org_member(org_id)
    AND created_by_user_id = auth.uid()
);

-- migrate:down
DROP TABLE IF EXISTS public.oauth_clients;
