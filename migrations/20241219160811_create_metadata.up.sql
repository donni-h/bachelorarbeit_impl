CREATE TYPE session_status AS ENUM ('open','complete','expired');

CREATE TABLE metadata (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL UNIQUE,
    username TEXT,
    status session_status,
    session_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_metadata_order FOREIGN KEY (order_id) REFERENCES orders (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);