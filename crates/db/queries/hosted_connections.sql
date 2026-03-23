--: HostedIntegration()
--: HostedConnectionSession()
--: HostedConnectionSessionContext()
--: CreatedHostedConnection()
--: PublicHostedIntegration()
--: DisconnectedHostedConnections()

--! get_system_integration_by_slug : HostedIntegration
SELECT
    i.id,
    COALESCE(i.slug, '') AS slug,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name,
    COALESCE(i.openapi_spec #>> '{info,description}', '') AS description,
    i.openapi_spec::TEXT AS openapi_spec
FROM public.integrations i
WHERE i.owner_kind = 'system'
  AND i.slug = :integration_slug::TEXT
LIMIT 1;

--! list_public_hosted_integrations : PublicHostedIntegration
SELECT
    i.id,
    COALESCE(i.slug, '') AS slug,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name,
    COALESCE(i.openapi_spec #>> '{info,description}', '') AS description,
    i.openapi_spec::TEXT AS openapi_spec,
    EXISTS(
        SELECT 1
        FROM public.integration_connections c
        WHERE c.org_id = public.b64url_to_uuid(:org_public_id::TEXT)
          AND c.integration_id = i.id
          AND c.end_user_id = :end_user_id::TEXT
          AND (
              c.visibility = 'org'
              OR c.created_by_user_id = auth.uid()
          )
    ) AS connected
FROM public.integrations i
WHERE i.owner_kind = 'system'
ORDER BY LOWER(COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled')), i.updated_at DESC;

--! create_hosted_connection_session : HostedConnectionSession
INSERT INTO public.hosted_connection_sessions (
    org_id,
    created_by_user_id,
    created_by_api_key_id,
    integration_id,
    integration_slug,
    end_user_id,
    end_user_name,
    end_user_email,
    suggested_connection_name,
    auth_type,
    token,
    expires_at
)
VALUES (
    :org_id::UUID,
    :created_by_user_id::UUID,
    :created_by_api_key_id::UUID,
    :integration_id::UUID,
    :integration_slug::TEXT,
    :end_user_id::TEXT,
    NULLIF(:end_user_name::TEXT, ''),
    NULLIF(:end_user_email::TEXT, ''),
    NULLIF(:suggested_connection_name::TEXT, ''),
    :auth_type::integration_auth_type,
    :token::TEXT,
    :expires_at::TIMESTAMPTZ
)
RETURNING
    token,
    expires_at;

--! get_hosted_connection_session : HostedConnectionSessionContext
SELECT
    s.id,
    public.uuid_to_b64url(s.org_id) AS org_public_id,
    s.integration_id,
    s.integration_slug,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS integration_name,
    u.issuer AS created_by_issuer,
    u.sub AS created_by_sub,
    s.end_user_id,
    COALESCE(s.end_user_name, '') AS end_user_name,
    COALESCE(s.end_user_email, '') AS end_user_email,
    COALESCE(s.suggested_connection_name, '') AS suggested_connection_name,
    s.auth_type::TEXT AS auth_type,
    s.expires_at,
    (s.expires_at <= NOW()) AS expired,
    (s.used_at IS NOT NULL) AS used
FROM public.hosted_connection_sessions s
INNER JOIN public.integrations i ON i.id = s.integration_id
INNER JOIN auth.users u ON u.id = s.created_by_user_id
WHERE s.token = :token::TEXT
LIMIT 1;

--! get_hosted_connection_session_for_update : HostedConnectionSessionContext
SELECT
    s.id,
    public.uuid_to_b64url(s.org_id) AS org_public_id,
    s.integration_id,
    s.integration_slug,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS integration_name,
    u.issuer AS created_by_issuer,
    u.sub AS created_by_sub,
    s.end_user_id,
    COALESCE(s.end_user_name, '') AS end_user_name,
    COALESCE(s.end_user_email, '') AS end_user_email,
    COALESCE(s.suggested_connection_name, '') AS suggested_connection_name,
    s.auth_type::TEXT AS auth_type,
    s.expires_at,
    (s.expires_at <= NOW()) AS expired,
    (s.used_at IS NOT NULL) AS used
FROM public.hosted_connection_sessions s
INNER JOIN public.integrations i ON i.id = s.integration_id
INNER JOIN auth.users u ON u.id = s.created_by_user_id
WHERE s.token = :token::TEXT
LIMIT 1
FOR UPDATE OF s;

--! create_api_key_integration_connection : CreatedHostedConnection
INSERT INTO public.integration_connections (
    org_id,
    integration_id,
    created_by_user_id,
    visibility,
    name,
    auth_type,
    api_key,
    end_user_id,
    end_user_name,
    end_user_email
)
VALUES (
    public.b64url_to_uuid(:org_public_id::TEXT),
    :integration_id::UUID,
    auth.uid(),
    'private'::resource_visibility,
    :name::TEXT,
    'api_key'::integration_auth_type,
    :api_key::TEXT,
    :end_user_id::TEXT,
    NULLIF(:end_user_name::TEXT, ''),
    NULLIF(:end_user_email::TEXT, '')
)
RETURNING
    id,
    name;

--! disconnect_public_hosted_integrations : DisconnectedHostedConnections
WITH deleted AS (
    DELETE FROM public.integration_connections c
    USING public.integrations i
    WHERE c.org_id = public.b64url_to_uuid(:org_public_id::TEXT)
      AND c.integration_id = i.id
      AND i.owner_kind = 'system'
      AND i.slug = :integration_slug::TEXT
      AND c.end_user_id = :end_user_id::TEXT
    RETURNING c.id
)
SELECT COUNT(*)::BIGINT AS deleted_count
FROM deleted;

--! mark_hosted_connection_session_used
UPDATE public.hosted_connection_sessions
SET
    used_at = NOW(),
    updated_at = NOW()
WHERE token = :token::TEXT
  AND used_at IS NULL;
