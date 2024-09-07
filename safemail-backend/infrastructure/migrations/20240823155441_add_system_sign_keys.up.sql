-- Add up migration script here
CREATE TABLE sm.system_sign_keys (
    private_key TEXT NOT NULL,
    public_key TEXT NOT NULL
);