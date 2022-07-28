-- works -------
---------------
---------------
CREATE TABLE work_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    INT NOT NULL,
    image       VARCHAR(500),
    count       INT NOT NULL
);

CREATE TABLE works (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    description VARCHAR,
    content     VARCHAR(30000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   BOOLEAN NOT NULL,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,

    CONSTRAINT fk_work_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX works_creator_idx ON works (user_id);

CREATE TABLE work_category (
    id                 SERIAL PRIMARY KEY,
    work_categories_id INT NOT NULL,
    work_id            INT NOT NULL,

   CONSTRAINT fk_work_category_cat
        FOREIGN KEY(work_categories_id)
            REFERENCES work_categories(id),

   CONSTRAINT fk_work_category_work
        FOREIGN KEY(work_id)
            REFERENCES works(id)
);

CREATE TABLE work_images (
    id   SERIAL PRIMARY KEY,
    work INT NOT NULL,
    src  VARCHAR(500) NOT NULL,

    CONSTRAINT fk_work_images
        FOREIGN KEY(work)
            REFERENCES works(id)
);
CREATE INDEX work_images_id_idx ON work_images (work);

CREATE TABLE work_videos (
    id   SERIAL PRIMARY KEY,
    work INT NOT NULL,
    src  VARCHAR(500) NOT NULL,

    CONSTRAINT fk_work_videos
        FOREIGN KEY(work)
            REFERENCES works(id)
);
CREATE INDEX work_videos_id_idx ON work_videos (work);
