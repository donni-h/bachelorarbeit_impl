CREATE TABLE order_item (
    id UUID PRIMARY KEY,
    product_name TEXT NOT NULL,
    item_id UUID NOT NULL,
    price NUMERIC(10,2) NOT NULL CHECK (price > 0),
    order_id UUID NOT NULL,
    CONSTRAINT fk_order_item_order FOREIGN KEY (order_id) REFERENCES orders (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);