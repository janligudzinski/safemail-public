CREATE SCHEMA sm;

CREATE TABLE sm.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (), username VARCHAR(128) UNIQUE NOT NULL, password TEXT NOT NULL, public_key TEXT UNIQUE NOT NULL
);