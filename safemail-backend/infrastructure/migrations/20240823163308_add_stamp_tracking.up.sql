-- Add up migration script here
CREATE TABLE sm.onetime_stamps (
    stamp_id UUID PRIMARY KEY,
    recipient_id UUID NOT NULL REFERENCES sm.users (id),
    used_or_revoked BOOLEAN NOT NULL DEFAULT false
);