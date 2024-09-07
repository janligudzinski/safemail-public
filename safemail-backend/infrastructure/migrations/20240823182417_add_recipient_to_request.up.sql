-- Add up migration script here
ALTER TABLE sm.onetime_stamp_requests
ADD COLUMN recipient_id UUID NOT NULL REFERENCES sm.users (id);