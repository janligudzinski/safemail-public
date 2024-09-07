-- Add up migration script here
ALTER TABLE sm.users ADD COLUMN public_verify_key TEXT NOT NULL;

ALTER TABLE sm.users
RENAME COLUMN public_key TO public_encryption_key;