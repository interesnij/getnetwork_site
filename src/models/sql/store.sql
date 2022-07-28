-- stores -------
---------------
---------------
CREATE TABLE store_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    position    INT NOT NULL,
    image       VARCHAR(500),
    count       INT NOT NULL
);

CREATE TABLE stores (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    description  VARCHAR,
    content      VARCHAR(30000),
    link         VARCHAR(500),
    image        VARCHAR(500),
    is_active    boolean NOT NULL,
    price        INT NOT NULL,
    user_id      INT NOT NULL,
    created      TIMESTAMP NOT NULL,

    CONSTRAINT fk_store_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX stores_creator_idx ON stores (user_id);

CREATE TABLE store_category (
    id                  SERIAL PRIMARY KEY,
    store_categories_id INT NOT NULL,
    store_id            INT NOT NULL,

   CONSTRAINT fk_store_category_cat
        FOREIGN KEY(store_categories_id)
            REFERENCES store_categories(id),

   CONSTRAINT fk_store_category_store
        FOREIGN KEY(store_id)
            REFERENCES stores(id)
);

CREATE TABLE store_images (
    id    SERIAL PRIMARY KEY,
    store INT NOT NULL,
    src   VARCHAR(500) NOT NULL,

    CONSTRAINT fk_store_images
        FOREIGN KEY(store)
            REFERENCES stores(id)
);
CREATE INDEX store_images_id_idx ON store_images (store);

CREATE TABLE store_videos (
    id    SERIAL PRIMARY KEY,
    store INT NOT NULL,
    src   VARCHAR(500) NOT NULL,

    CONSTRAINT fk_store_videos
        FOREIGN KEY(store)
            REFERENCES stores(id)
);
CREATE INDEX store_videos_id_idx ON store_videos (store);
