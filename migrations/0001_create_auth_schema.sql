CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('cortex', 'partner', 'user')),

    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'suspended', 'deleted')),

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX organizations_name_unique_idx
    ON organizations (lower(name));


CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    email TEXT,
    wallet_address TEXT,

    social_provider TEXT,
    social_provider_user_id TEXT,

    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'suspended', 'deleted')),

    key_limit INTEGER NOT NULL DEFAULT 2
        CHECK (key_limit >= 0),

    rate_limit_tier TEXT NOT NULL DEFAULT 'free'
        CHECK (rate_limit_tier IN ('free', 'paid', 'internal')),

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT users_identity_required_chk CHECK (
        email IS NOT NULL
        OR wallet_address IS NOT NULL
        OR (
            social_provider IS NOT NULL
            AND social_provider_user_id IS NOT NULL
        )
    )
);

CREATE UNIQUE INDEX users_email_unique_idx
    ON users (lower(email))
    WHERE email IS NOT NULL;

CREATE UNIQUE INDEX users_wallet_address_unique_idx
    ON users (lower(wallet_address))
    WHERE wallet_address IS NOT NULL;

CREATE UNIQUE INDEX users_social_identity_unique_idx
    ON users (social_provider, social_provider_user_id)
    WHERE social_provider IS NOT NULL
      AND social_provider_user_id IS NOT NULL;


CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    owner_type TEXT NOT NULL CHECK (owner_type IN ('organization', 'user')),

    organization_id UUID
        REFERENCES organizations(id)
        ON DELETE CASCADE,

    user_id UUID
        REFERENCES users(id)
        ON DELETE CASCADE,

    name TEXT NOT NULL,

    key_prefix TEXT NOT NULL UNIQUE,
    key_hash TEXT NOT NULL,

    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'revoked')),

    rate_limit_per_minute INTEGER NOT NULL DEFAULT 60
        CHECK (rate_limit_per_minute > 0),

    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT api_keys_single_owner_chk CHECK (
        (
            owner_type = 'organization'
            AND organization_id IS NOT NULL
            AND user_id IS NULL
        )
        OR
        (
            owner_type = 'user'
            AND organization_id IS NULL
            AND user_id IS NOT NULL
        )
    )
);

CREATE INDEX api_keys_organization_id_idx
    ON api_keys (organization_id);

CREATE INDEX api_keys_user_id_idx
    ON api_keys (user_id);

CREATE INDEX api_keys_key_prefix_idx
    ON api_keys (key_prefix);

CREATE INDEX api_keys_status_idx
    ON api_keys (status);

CREATE INDEX api_keys_owner_type_idx
    ON api_keys (owner_type);


CREATE TABLE api_key_scopes (
    api_key_id UUID NOT NULL
        REFERENCES api_keys(id)
        ON DELETE CASCADE,

    scope TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (api_key_id, scope)
);

CREATE INDEX api_key_scopes_scope_idx
    ON api_key_scopes (scope);