
-- blogs -------
---------------
---------------
CREATE TABLE blog_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    SMALLINT NOT NULL,
    image       VARCHAR(500),
    count       SMALLINT NOT NULL
);

CREATE TABLE blogs (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    content     VARCHAR(30000),
    link        VARCHAR(500),
    image       VARCHAR(500),
    is_active   BOOLEAN NOT NULL,
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
