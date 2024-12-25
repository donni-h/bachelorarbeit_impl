CREATE TYPE session_status AS ENUM ('open','complete','expired');

CREATE TABLE order_details (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    status session_status,
    session_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);