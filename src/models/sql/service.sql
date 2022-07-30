
-- services -------
---------------
---------------
CREATE TABLE service_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    SMALLINT NOT NULL,
    image       VARCHAR(500),
    count       SMALLINT NOT NULL
);

CREATE TABLE services (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    description VARCHAR,
    content     VARCHAR(30000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   BOOLEAN NOT NULL,
    price       INT NOT NULL,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,
    position    SMALLINT NOT NULL,

    CONSTRAINT fk_service_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX services_creator_idx ON services (user_id);

CREATE TABLE service_category (
    id                    SERIAL PRIMARY KEY,
    service_categories_id INT NOT NULL,
    service_id            INT NOT NULL,

   CONSTRAINT fk_service_category_cat
        FOREIGN KEY(service_categories_id)
            REFERENCES service_categories(id),

   CONSTRAINT fk_service_category_service
        FOREIGN KEY(service_id)
            REFERENCES services(id)
);

CREATE TABLE service_images (
    id      SERIAL PRIMARY KEY,
    service INT NOT NULL,
    src     VARCHAR(500) NOT NULL,

    CONSTRAINT fk_service_images
        FOREIGN KEY(service)
            REFERENCES services(id)
);
CREATE INDEX service_images_id_idx ON service_images (service);

CREATE TABLE service_videos (
    id      SERIAL PRIMARY KEY,
    service INT NOT NULL,
    src     VARCHAR(500) NOT NULL,

    CONSTRAINT fk_service_videos
        FOREIGN KEY(service)
            REFERENCES services(id)
);
CREATE INDEX service_videos_id_idx ON service_videos (service);
