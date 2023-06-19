CREATE TABLE users (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    username TEXT NOT NULL UNIQUE,
    time_updated timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC'),
    time_created  timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC')
);

CREATE TABLE user_auth (
    user_id uuid NOT NULL,
    password_hash TEXT NOT NULL,
    time_updated timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC'),
    time_created timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC')
);

CREATE TABLE organization (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    time_updated timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC'),
    time_created timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC')
);

CREATE TABLE role_access (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    user_id uuid NOT NULL,
    resource TEXT NOT NULL,
    role TEXT NOT NULL,
    time_created timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC'),
    time_updated timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC')
);

CREATE TABLE user_organization(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    user_id uuid NOT NULL,
    organization_id UUID NOT NULL,
    time_created timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC'),
    time_updated timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'UTC')
);