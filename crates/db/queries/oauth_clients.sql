--: OAuthClientCard()
--: OAuthClientMutation()

--! list_oauth_clients : OAuthClientCard
SELECT
    id,
    provider,
    client_id,
    created_at
FROM public.oauth_clients
WHERE org_id = public.b64url_to_uuid(:org_id::TEXT)
ORDER BY LOWER(provider), LOWER(client_id), created_at DESC;

--! create_oauth_client : OAuthClientMutation
WITH inserted AS (
    INSERT INTO public.oauth_clients (
        org_id,
        created_by_user_id,
        provider,
        client_id,
        client_secret
    )
    VALUES (
        public.b64url_to_uuid(:org_id::TEXT),
        auth.uid(),
        :provider::TEXT,
        :client_id::TEXT,
        :client_secret::TEXT
    )
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM inserted) AS changed;
