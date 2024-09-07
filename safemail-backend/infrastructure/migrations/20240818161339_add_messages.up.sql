-- Add up migration script here
CREATE TABLE sm.messages (
    id BIGSERIAL PRIMARY KEY,
    recipient_id UUID NOT NULL,
    metadata TEXT NOT NULL,
    recipient_metadata TEXT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (recipient_id) REFERENCES sm.users (id),
    CONSTRAINT valid_base64_metadata CHECK (
        metadata ~ '^[A-Za-z0-9+/]*={0,2}$'
    ),
    CONSTRAINT valid_base64_recipient_metadata CHECK (
        recipient_metadata ~ '^[A-Za-z0-9+/]*={0,2}$'
    ),
    CONSTRAINT valid_base64_content CHECK (
        content ~ '^[A-Za-z0-9+/]*={0,2}$'
    )
);

-- Index for faster queries on recipient
CREATE INDEX idx_messages_recipient ON sm.messages (recipient_id);