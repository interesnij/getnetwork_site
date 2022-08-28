-- orders -------
---------------
---------------
CREATE TABLE orders (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    types       SMALLINT NOT NULL, -- 1 услуга, 2 товар, 3 работа
    object_id   INT NOT NULL,
    username    VARCHAR(200) NOT NULL,
    email       VARCHAR(200) NOT NULL,
    description VARCHAR(1000),
    created     TIMESTAMP NOT NULL
);

CREATE TABLE order_files (
    id       SERIAL PRIMARY KEY,
    order_id INT NOT NULL,
    src      VARCHAR(500) NOT NULL,

    CONSTRAINT fk_order_files
        FOREIGN KEY(order_id)
            REFERENCES orders(id)
);
CREATE INDEX order_files_id_idx ON order_files (order_id);
