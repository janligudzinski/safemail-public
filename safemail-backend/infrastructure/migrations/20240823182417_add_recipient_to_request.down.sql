-- Add down migration script here
ALTER TABLE sm.onetime_stamp_requests DROP COLUMN recipient_id;