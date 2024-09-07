CREATE TABLE sm.sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid (), user_id UUID NOT NULL REFERENCES sm.users (id), active BOOLEAN NOT NULL DEFAULT false, challenge_string VARCHAR(128) NOT NULL, requested_at_utc TIMESTAMPTZ NOT NULL, activated_at_utc TIMESTAMPTZ, expires_at_utc TIMESTAMPTZ NOT NULL
);