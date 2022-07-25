-- feedback -------
---------------
---------------
CREATE TABLE feedbacks (
    id       SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(200) NOT NULL,
    message  VARCHAR(1000) NOT NULL
);

-- users -------
---------------
---------------
CREATE TABLE users (
    id       SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(100) NOT NULL,
    password VARCHAR(1000) NOT NULL, 
    bio      VARCHAR(500),
    image    VARCHAR(500),
    perm     SMALLINT NOT NULL,

    UNIQUE(username),
    UNIQUE(email)
);

INSERT INTO users(id, username, email, password, perm)
VALUES (1, 'Serg', 'ochkarik1983@mail.ru', 'ulihos46', 60)
ON CONFLICT DO NOTHING;

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
    content     VARCHAR(10000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   boolean NOT NULL,
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


-- blogs -------
---------------
---------------
CREATE TABLE blog_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    INT NOT NULL,
    image       VARCHAR(500),
    count       INT NOT NULL
);

CREATE TABLE blogs (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    content     VARCHAR(10000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   boolean NOT NULL,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,

    CONSTRAINT fk_blog_creator_2
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX blogs_creator_idx ON blogs (user_id);

CREATE TABLE blog_comments (
    id        SERIAL PRIMARY KEY,
    comment   VARCHAR(1000) NOT NULL,
    blog_id   INT NOT NULL,
    user_id   INT NOT NULL,
    parent_id INT,
    created   TIMESTAMP NOT NULL,

    CONSTRAINT fk_blog_comment
        FOREIGN KEY(blog_id)
            REFERENCES blogs(id),

    CONSTRAINT fk_user_blog_comment
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_blog_parent_comment
        FOREIGN KEY(parent_id)
            REFERENCES blog_comments(id)
);
CREATE INDEX blog_comments_id_idx ON blog_comments (blog_id);
CREATE INDEX blog_comments_user_id_idx ON blog_comments (user_id);

CREATE TABLE blog_category (
    id                 SERIAL PRIMARY KEY,
    blog_categories_id INT NOT NULL,
    blog_id            INT NOT NULL,

   CONSTRAINT fk_blog_category_cat
        FOREIGN KEY(blog_categories_id)
            REFERENCES blog_categories(id),

   CONSTRAINT fk_blog_category_blog
        FOREIGN KEY(blog_id)
            REFERENCES blogs(id)
);

CREATE TABLE blog_images (
    id   SERIAL PRIMARY KEY,
    blog INT NOT NULL,
    src  VARCHAR(500) NOT NULL,

    CONSTRAINT fk_blog_images
        FOREIGN KEY(blog)
            REFERENCES blogs(id)
);

CREATE INDEX blog_images_id_idx ON blog_images (blog);

CREATE TABLE blog_videos (
    id   SERIAL PRIMARY KEY,
    blog INT NOT NULL,
    src  VARCHAR(500) NOT NULL,

    CONSTRAINT fk_blog_videos
        FOREIGN KEY(blog)
            REFERENCES blogs(id)
);
CREATE INDEX blog_videos_id_idx ON blog_videos (blog);

-- services -------
---------------
---------------
CREATE TABLE service_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    INT NOT NULL,
    image       VARCHAR(500),
    count       INT NOT NULL
);

CREATE TABLE services (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    description VARCHAR,
    content     VARCHAR(10000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   boolean NOT NULL,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,

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

-- serve -------
---------------
---------------
CREATE TABLE tech_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    INT NOT NULL,
    count       INT NOT NULL,
    user_id     INT NOT NULL,
);

CREATE TABLE serve_categories (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    description     VARCHAR(500),
    cat_name        VARCHAR(100) NOT NULL,
    tech_categories INT NOT NULL,
    position        INT NOT NULL,
    count           INT NOT NULL,
    default_price   INT default 0,
    user_id         INT NOT NULL,

    CONSTRAINT fk_tech_category
        FOREIGN KEY(tech_categories)
            REFERENCES serve_categories(id)
);

CREATE TABLE serve (
    id               SERIAL PRIMARY KEY,
    name             VARCHAR(100) NOT NULL,
    cat_name         VARCHAR(100) NOT NULL,
    description      VARCHAR(500),
    position         INT NOT NULL,
    serve_categories INT NOT NULL,
    price            INT,
    price_acc        INT,
    social_price     INT,
    man_hours        INT,
    is_default       boolean not null default false,
    user_id          INT NOT NULL,

    CONSTRAINT fk_serve_category
        FOREIGN KEY(serve_categories)
            REFERENCES serve(id),
    CONSTRAINT fk_serve_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

CREATE TABLE serve_items (
    id         SERIAL PRIMARY KEY,
    serve_id   INT NOT NULL,
    service_id INT NOT NULL,
    store_id   INT NOT NULL,
    work_id    INT NOT NULL
);

-- stores -------
---------------
---------------
CREATE TABLE store_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    INT NOT NULL,
    image       VARCHAR(500),
    count       INT NOT NULL
);

CREATE TABLE stores (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    description  VARCHAR,
    content      VARCHAR(10000),
    link         VARCHAR(500),
    image        VARCHAR(500),
    is_active    boolean NOT NULL,
    price        INT,
    price_acc    INT,
    social_price INT,
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

-- wikis -------
---------------
---------------
CREATE TABLE wiki_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    INT NOT NULL,
    image       VARCHAR(500),
    count       INT NOT NULL
);

CREATE TABLE wikis (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    description VARCHAR,
    content     VARCHAR(10000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   boolean NOT NULL,
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

-- tags -------
---------------
---------------
CREATE TABLE tags (
    id            SERIAL PRIMARY KEY,
    name          VARCHAR(100) NOT NULL,
    position      INT NOT NULL,

    count         INT NOT NULL,
    blog_count    INT NOT NULL,
    service_count INT NOT NULL,
    store_count   INT NOT NULL,
    wiki_count    INT NOT NULL,
    work_count    INT NOT NULL,

    user_id       INT NOT NULL,

    CONSTRAINT fk_tag_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);


CREATE TABLE tags_items (
    id         SERIAL PRIMARY KEY,
    tag_id     INT NOT NULL,
    service_id INT NOT NULL,
    store_id   INT NOT NULL,
    blog_id    INT NOT NULL,
    wiki_id    INT NOT NULL,
    work_id    INT NOT NULL,
    created    TIMESTAMP NOT NULL
);
