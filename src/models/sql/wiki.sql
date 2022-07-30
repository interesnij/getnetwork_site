-- wikis -------
---------------
---------------
CREATE TABLE wiki_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    position    SMALLINT NOT NULL,
    image       VARCHAR(500),
    count       SMALLINT NOT NULL
);

CREATE TABLE wikis (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    description VARCHAR,
    content     VARCHAR(30000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   BOOLEAN NOT NULL,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,

    CONSTRAINT fk_wiki_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX wikis_creator_idx ON wikis (user_id);

CREATE TABLE wiki_category (
    id                 SERIAL PRIMARY KEY,
    wiki_categories_id INT NOT NULL,
    wiki_id            INT NOT NULL,

   CONSTRAINT fk_wiki_category_cat
        FOREIGN KEY(wiki_categories_id)
            REFERENCES wiki_categories(id),

   CONSTRAINT fk_wiki_category_wiki
        FOREIGN KEY(wiki_id)
            REFERENCES wikis(id)
);

CREATE TABLE wiki_images (
    id   SERIAL PRIMARY KEY,
    wiki INT NOT NULL,
    src  VARCHAR(500) NOT NULL,

    CONSTRAINT fk_wiki_images
        FOREIGN KEY(wiki)
            REFERENCES wikis(id)
);
CREATE INDEX wiki_images_id_idx ON wiki_images (wiki);

CREATE TABLE wiki_videos (
    id   SERIAL PRIMARY KEY,
    wiki INT NOT NULL,
    src  VARCHAR(500) NOT NULL,

    CONSTRAINT fk_wiki_videos
        FOREIGN KEY(wiki)
            REFERENCES wikis(id)
);
CREATE INDEX wiki_videos_id_idx ON wiki_videos (wiki);
