-- migrate:up

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Operations'::text), true),
            '{info,x-developer}',
            to_jsonb('Airtable'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://airtable.com'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://support.airtable.com'::text),
    true
)
WHERE slug = 'airtable';

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Data & Analytics'::text), true),
            '{info,x-developer}',
            to_jsonb('Apollo.io'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://www.apollo.io'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://knowledge.apollo.io'::text),
    true
)
WHERE slug = 'apollo';

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Finance'::text), true),
            '{info,x-developer}',
            to_jsonb('Blockchain.com'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://www.blockchain.com'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://support.blockchain.com'::text),
    true
)
WHERE slug = 'blockchain';

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Operations'::text), true),
            '{info,x-developer}',
            to_jsonb('Dropbox'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://www.dropbox.com'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://help.dropbox.com'::text),
    true
)
WHERE slug = 'dropbox';

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Communication'::text), true),
            '{info,x-developer}',
            to_jsonb('Google'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://workspace.google.com/products/calendar/'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://support.google.com/calendar'::text),
    true
)
WHERE slug = 'gcal';

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Communication'::text), true),
            '{info,x-developer}',
            to_jsonb('Google'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://workspace.google.com/products/contacts/'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://support.google.com/contacts'::text),
    true
)
WHERE slug = 'gpeople';

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Operations'::text), true),
            '{info,x-developer}',
            to_jsonb('Google'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://workspace.google.com/products/drive/'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://support.google.com/drive'::text),
    true
)
WHERE slug = 'gdrive';

UPDATE public.integrations
SET openapi_spec = jsonb_set(
    jsonb_set(
        jsonb_set(
            jsonb_set(openapi_spec, '{info,x-category}', to_jsonb('Data & Analytics'::text), true),
            '{info,x-developer}',
            to_jsonb('Serper'::text),
            true
        ),
        '{info,x-website}',
        to_jsonb('https://serper.dev'::text),
        true
    ),
    '{info,x-support}',
    to_jsonb('https://serper.dev'::text),
    true
)
WHERE slug = 'search-serper-search';

-- migrate:down

UPDATE public.integrations
SET openapi_spec = openapi_spec #- '{info,x-category}' #- '{info,x-developer}' #- '{info,x-website}' #- '{info,x-support}'
WHERE owner_kind = 'system';
