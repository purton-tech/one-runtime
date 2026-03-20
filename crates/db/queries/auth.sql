--: User()

--: AuthUser()
--: EnsureOrgMembership()
--: UserOrg()
--: ApiKeyLookup()
--: CreatedApiKey()

--! upsert_user_by_issuer_sub (first_name?, last_name?) : AuthUser
INSERT INTO auth.users (
    issuer,
    sub,
    email,
    first_name,
    last_name
)
VALUES (
    :issuer::TEXT,
    :sub::TEXT,
    :email::TEXT,
    :first_name::TEXT,
    :last_name::TEXT
)
ON CONFLICT (issuer, sub)
DO UPDATE SET
    email = EXCLUDED.email,
    first_name = EXCLUDED.first_name,
    last_name = EXCLUDED.last_name,
    updated_at = NOW()
RETURNING
    id,
    issuer,
    sub,
    email;

--! get_current_user : AuthUser
SELECT
    id,
    issuer,
    sub,
    email
FROM auth.users
WHERE id = auth.uid();

--! ensure_default_org_membership_for_user : EnsureOrgMembership
WITH has_membership AS (
    SELECT 1
    FROM org.org_memberships
    WHERE user_id = :user_id::UUID
    LIMIT 1
),
inserted_org AS (
    INSERT INTO org.orgs (name)
    SELECT :org_name::TEXT
    WHERE NOT EXISTS (SELECT 1 FROM has_membership)
    RETURNING id
),
inserted_membership AS (
    INSERT INTO org.org_memberships (org_id, user_id, role)
    SELECT io.id, :user_id::UUID, 'owner'::org.org_role
    FROM inserted_org io
    ON CONFLICT (org_id, user_id) DO NOTHING
    RETURNING 1
)
SELECT EXISTS(SELECT 1 FROM inserted_membership) AS ensured;

--! get_first_org_for_user : UserOrg
SELECT
    org_id,
    public.uuid_to_b64url(org_id) AS org_public_id
FROM org.org_memberships
WHERE user_id = :user_id::UUID
ORDER BY joined_at ASC
LIMIT 1;

--! set_request_claim_sub
SELECT set_config(
    'request.jwt.claim.sub',
    :claim_sub::TEXT,
    true
);

--! set_request_claim_iss
SELECT set_config(
    'request.jwt.claim.iss',
    :claim_iss::TEXT,
    true
);

--! get_users : User
SELECT 
    id, 
    email
FROM auth.users;

--! create_api_key : CreatedApiKey
INSERT INTO auth.api_keys (
    user_id,
    org_id,
    label,
    key_prefix,
    secret_hash
)
VALUES (
    :user_id::UUID,
    public.b64url_to_uuid(:org_id::TEXT),
    :label::TEXT,
    :key_prefix::TEXT,
    :secret_hash::TEXT
)
RETURNING
    id,
    key_prefix;

--! get_api_key_for_auth : ApiKeyLookup
SELECT
    ak.id,
    ak.user_id,
    ak.org_id,
    public.uuid_to_b64url(ak.org_id) AS org_public_id,
    ak.label,
    ak.key_prefix,
    ak.secret_hash,
    u.issuer,
    u.sub,
    u.email
FROM auth.api_keys ak
INNER JOIN auth.users u ON u.id = ak.user_id
WHERE ak.key_prefix = :key_prefix::TEXT
  AND ak.revoked_at IS NULL
LIMIT 1;

--! touch_api_key_last_used
UPDATE auth.api_keys
SET last_used_at = NOW()
WHERE id = :api_key_id::UUID;
