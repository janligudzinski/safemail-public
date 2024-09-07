-- Add up migration script here
CREATE TABLE sm.onetime_stamp_requests (
    stamp_request_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    difficulty BIGINT NOT NULL,
    valid_to TIMESTAMPTZ NOT NULL DEFAULT(NOW() + INTERVAL '15 minutes'),
    solved_at TIMESTAMPTZ
);