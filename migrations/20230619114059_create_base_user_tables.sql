CREATE TABLE user_auth (
    user_id INT PRIMARY KEY,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE organization (
    id INT PRIMARY KEY,
    name TEXT,
    short_name TEXT,
    time_updated INT,
    time_created INT
);

CREATE TABLE role_access (
    id INT PRIMARY KEY,
    user_id INT NOT NULL,
    resource TEXT NOT NULL,
    role TEXT NOT NULL,
    time_created INT NOT NULL,
    time_updated INT NOT NULL
);
