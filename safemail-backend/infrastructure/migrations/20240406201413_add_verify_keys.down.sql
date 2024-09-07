-- Add down migration script here
ALTER TABLE sm.users
RENAME COLUMN public_encryption_key TO public_key;

ALTER TABLE sm.users DROP COLUMN public_verify_key;