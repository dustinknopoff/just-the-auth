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

CREATE TABLE role (
    id UUID NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE user_role (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL
);
