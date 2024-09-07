-- Add down migration script here
ALTER TABLE sm.users
ADD COLUMN password TEXT NOT NULL DEFAULT 'password';