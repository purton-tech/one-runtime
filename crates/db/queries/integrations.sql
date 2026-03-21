--: IntegrationCard()
--: IntegrationForm()
--: IntegrationMutation()

--! list_integrations : IntegrationCard
SELECT
    i.id,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name,
    COALESCE(i.openapi_spec #>> '{info,description}', '') AS description,
    i.owner_kind::TEXT AS owner_kind,
    i.visibility::TEXT AS visibility,
    CASE
        WHEN i.owner_kind = 'org' THEN (
            (i.visibility = 'private' AND i.created_by_user_id = auth.uid())
            OR (i.visibility = 'org' AND org.is_org_admin(i.org_id))
        )
        ELSE FALSE
    END AS can_manage,
    i.updated_at
FROM public.integrations i
WHERE i.owner_kind = 'system'
   OR (
      i.owner_kind = 'org'
      AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
      AND (
          i.visibility = 'org'
          OR i.created_by_user_id = auth.uid()
      )
  )
ORDER BY (i.owner_kind = 'system') DESC, LOWER(COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled')), i.updated_at DESC;

--! get_integration_for_edit : IntegrationForm
SELECT
    i.id,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name,
    COALESCE(i.openapi_spec #>> '{info,description}', '') AS description,
    i.visibility::TEXT AS visibility,
    i.openapi_spec::TEXT AS openapi_spec
FROM public.integrations i
WHERE i.id = :id::UUID
  AND i.owner_kind = 'org'
  AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
LIMIT 1;

--! create_integration : IntegrationMutation
WITH inserted AS (
    INSERT INTO public.integrations (
        owner_kind,
        org_id,
        created_by_user_id,
        visibility,
        openapi_spec
    )
    VALUES (
        'org'::integration_owner_kind,
        public.b64url_to_uuid(:org_id::TEXT),
        auth.uid(),
        :visibility::resource_visibility,
        :openapi_spec::JSONB
    )
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM inserted) AS changed;

--! update_integration : IntegrationMutation
WITH updated AS (
    UPDATE public.integrations i
    SET
        visibility = :visibility::resource_visibility,
        openapi_spec = :openapi_spec::JSONB,
        updated_at = NOW()
    WHERE i.id = :id::UUID
      AND i.owner_kind = 'org'
      AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM updated) AS changed;

--! delete_integration : IntegrationMutation
WITH deleted AS (
    DELETE FROM public.integrations i
    WHERE i.id = :id::UUID
      AND i.owner_kind = 'org'
      AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM deleted) AS changed;
