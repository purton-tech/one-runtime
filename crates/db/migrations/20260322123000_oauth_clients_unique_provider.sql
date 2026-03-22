-- migrate:up
WITH ranked AS (
    SELECT
        id,
        ROW_NUMBER() OVER (
            PARTITION BY org_id, LOWER(BTRIM(provider))
            ORDER BY updated_at DESC, created_at DESC, id DESC
        ) AS row_num
    FROM public.oauth_clients
),
deleted AS (
    DELETE FROM public.oauth_clients oc
    USING ranked r
    WHERE oc.id = r.id
      AND r.row_num > 1
    RETURNING oc.id
)
UPDATE public.oauth_clients
SET
    provider = LOWER(BTRIM(provider)),
    updated_at = NOW()
WHERE provider <> LOWER(BTRIM(provider))
   OR provider <> BTRIM(provider);

ALTER TABLE public.oauth_clients
    DROP CONSTRAINT oauth_clients_org_id_provider_client_id_key;

CREATE UNIQUE INDEX oauth_clients_org_provider_unique_idx
    ON public.oauth_clients (org_id, LOWER(provider));

-- migrate:down
DROP INDEX IF EXISTS oauth_clients_org_provider_unique_idx;

ALTER TABLE public.oauth_clients
    ADD CONSTRAINT oauth_clients_org_id_provider_client_id_key
    UNIQUE (org_id, provider, client_id);
