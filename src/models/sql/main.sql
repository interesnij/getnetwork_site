-- feedback -------
---------------
---------------
CREATE TABLE feedbacks (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    message TEXT
);

-- users -------
---------------
---------------
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    bio TEXT,
    image TEXT,

    UNIQUE(username),
    UNIQUE(email)
);

-- works -------
---------------
---------------
CREATE TABLE work_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    work_position INT,
    image TEXT,
    work_count INT
);

CREATE TABLE works (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    content TEXT,
    link TEXT,
    image TEXT,
    is_work_active boolean NOT NULL,
    creator INT NOT NULL,
    work_created TIMESTAMP NOT NULL,

    CONSTRAINT fk_work_creator
        FOREIGN KEY(creator)
            REFERENCES users(id)
);
CREATE INDEX works_creator_idx ON works (creator);

CREATE TABLE work_category (
    id SERIAL PRIMARY KEY,
    work_categories_id INT,
    work_id INT,

   CONSTRAINT fk_work_category_cat
        FOREIGN KEY(work_categories_id)
            REFERENCES work_categories(id),

   CONSTRAINT fk_work_category_work
        FOREIGN KEY(work_id)
            REFERENCES works(id)
);

CREATE TABLE work_images (
    id SERIAL PRIMARY KEY,
    work INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_work_images
        FOREIGN KEY(work)
            REFERENCES works(id)
);
CREATE INDEX work_images_id_idx ON work_images (work);

CREATE TABLE work_videos (
    id SERIAL PRIMARY KEY,
    work INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_work_videos
        FOREIGN KEY(work)
            REFERENCES works(id)
);
CREATE INDEX work_videos_id_idx ON work_videos (work);


-- blogs -------
---------------
---------------
CREATE TABLE blog_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    blog_position INT,
    image TEXT,
    blog_count INT
);

CREATE TABLE blogs (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    content TEXT,
    link TEXT,
    image TEXT,
    is_blog_active boolean NOT NULL,
    creator INT NOT NULL,
    blog_created TIMESTAMP NOT NULL,

    CONSTRAINT fk_blog_creator_2
        FOREIGN KEY(creator)
            REFERENCES users(id)
);
CREATE INDEX blogs_creator_idx ON blogs (creator);

CREATE TABLE blog_comments (
    id SERIAL PRIMARY KEY,
    comment VARCHAR NOT NULL,
    blog_id INT NOT NULL,
    user_id INT NOT NULL,
    parent_comment_id INT,
    created_at TIMESTAMP NOT NULL,

    CONSTRAINT fk_blog_comment
        FOREIGN KEY(blog_id)
            REFERENCES blogs(id),

    CONSTRAINT fk_user_blog_comment
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_blog_parent_comment
        FOREIGN KEY(parent_comment_id)
            REFERENCES blog_comments(id)
);
CREATE INDEX blog_comments_id_idx ON blog_comments (blog_id);
CREATE INDEX blog_comments_user_id_idx ON blog_comments (user_id);

CREATE TABLE blog_category (
    id SERIAL PRIMARY KEY,
    blog_categories_id INT,
    blog_id INT,

   CONSTRAINT fk_blog_category_cat
        FOREIGN KEY(blog_categories_id)
            REFERENCES blog_categories(id),

   CONSTRAINT fk_blog_category_blog
        FOREIGN KEY(blog_id)
            REFERENCES blogs(id)
);

CREATE TABLE blog_images (
    id SERIAL PRIMARY KEY,
    blog INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_blog_images
        FOREIGN KEY(blog)
            REFERENCES blogs(id)
);

CREATE INDEX blog_images_id_idx ON blog_images (blog);

CREATE TABLE blog_videos (
    id SERIAL PRIMARY KEY,
    blog INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_blog_videos
        FOREIGN KEY(blog)
            REFERENCES blogs(id)
);
CREATE INDEX blog_videos_id_idx ON blog_videos (blog);

-- services -------
---------------
---------------
CREATE TABLE service_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    service_position INT,
    image TEXT,
    service_count INT
);

CREATE TABLE services (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    content TEXT,
    link TEXT,
    image TEXT,
    is_service_active boolean NOT NULL,
    creator INT NOT NULL,
    service_created TIMESTAMP NOT NULL,

    CONSTRAINT fk_service_creator
        FOREIGN KEY(creator)
            REFERENCES users(id)
);
CREATE INDEX services_creator_idx ON services (creator);

CREATE TABLE service_category (
    id SERIAL PRIMARY KEY,
    service_categories_id INT,
    service_id INT,

   CONSTRAINT fk_service_category_cat
        FOREIGN KEY(service_categories_id)
            REFERENCES service_categories(id),

   CONSTRAINT fk_service_category_service
        FOREIGN KEY(service_id)
            REFERENCES services(id)
);

CREATE TABLE service_images (
    id SERIAL PRIMARY KEY,
    service INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_service_images
        FOREIGN KEY(service)
            REFERENCES services(id)
);
CREATE INDEX service_images_id_idx ON service_images (service);

CREATE TABLE service_videos (
    id SERIAL PRIMARY KEY,
    service INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_service_videos
        FOREIGN KEY(service)
            REFERENCES services(id)
);
CREATE INDEX service_videos_id_idx ON service_videos (service);

-- serve -------
---------------
---------------
CREATE TABLE tech_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    tech_position INT,
    tech_count INT
);

CREATE TABLE serve_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    cat_name VARCHAR NOT NULL,
    tech_categories INT,
    serve_position INT,
    serve_count INT,
    default_price INT default 0,

    CONSTRAINT fk_tech_category
        FOREIGN KEY(tech_categories)
            REFERENCES serve_categories(id)
);

CREATE TABLE serve (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    cat_name VARCHAR NOT NULL,
    description TEXT,
    serve_position INT,
    serve_categories INT,
    price INT,
    price_acc INT,
    social_price INT,
    man_hours INT,
    is_default boolean not null default false,

    CONSTRAINT fk_serve_category
        FOREIGN KEY(serve_categories)
            REFERENCES serve(id)
);

CREATE TABLE serve_items (
    id SERIAL PRIMARY KEY,
    serve_id INT,
    service_id INT,
    store_id INT,
    work_id INT
);

-- stores -------
---------------
---------------
CREATE TABLE store_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    store_position INT,
    image TEXT,
    store_count INT
);

CREATE TABLE stores (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    content TEXT,
    link TEXT,
    image TEXT,
    is_store_active boolean NOT NULL,
    price INT,
    price_acc INT,
    social_price INT,
    creator INT NOT NULL,
    store_created TIMESTAMP NOT NULL,

    CONSTRAINT fk_store_creator
        FOREIGN KEY(creator)
            REFERENCES users(id)
);
CREATE INDEX stores_creator_idx ON stores (creator);

CREATE TABLE store_category (
    id SERIAL PRIMARY KEY,
    store_categories_id INT,
    store_id INT,

   CONSTRAINT fk_store_category_cat
        FOREIGN KEY(store_categories_id)
            REFERENCES store_categories(id),

   CONSTRAINT fk_store_category_store
        FOREIGN KEY(store_id)
            REFERENCES stores(id)
);

CREATE TABLE store_images (
    id SERIAL PRIMARY KEY,
    store INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_store_images
        FOREIGN KEY(store)
            REFERENCES stores(id)
);
CREATE INDEX store_images_id_idx ON store_images (store);

CREATE TABLE store_videos (
    id SERIAL PRIMARY KEY,
    store INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_store_videos
        FOREIGN KEY(store)
            REFERENCES stores(id)
);
CREATE INDEX store_videos_id_idx ON store_videos (store);

-- wikis -------
---------------
---------------
CREATE TABLE wiki_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    wiki_position INT,
    image TEXT,
    wiki_count INT
);

CREATE TABLE wikis (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    content TEXT,
    link TEXT,
    image TEXT,
    is_wiki_active boolean NOT NULL,
    creator INT NOT NULL,
    wiki_created TIMESTAMP NOT NULL,

    CONSTRAINT fk_wiki_creator
        FOREIGN KEY(creator)
            REFERENCES users(id)
);
CREATE INDEX wikis_creator_idx ON wikis (creator);

CREATE TABLE wiki_category (
    id SERIAL PRIMARY KEY,
    wiki_categories_id INT,
    wiki_id INT,

   CONSTRAINT fk_wiki_category_cat
        FOREIGN KEY(wiki_categories_id)
            REFERENCES wiki_categories(id),

   CONSTRAINT fk_wiki_category_wiki
        FOREIGN KEY(wiki_id)
            REFERENCES wikis(id)
);

CREATE TABLE wiki_images (
    id SERIAL PRIMARY KEY,
    wiki INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_wiki_images
        FOREIGN KEY(wiki)
            REFERENCES wikis(id)
);
CREATE INDEX wiki_images_id_idx ON wiki_images (wiki);

CREATE TABLE wiki_videos (
    id SERIAL PRIMARY KEY,
    wiki INT NOT NULL,
    src TEXT NOT NULL,

    CONSTRAINT fk_wiki_videos
        FOREIGN KEY(wiki)
            REFERENCES wikis(id)
);
CREATE INDEX wiki_videos_id_idx ON wiki_videos (wiki);

-- tags -------
---------------
---------------
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    tag_position INT,
    tag_count INT,
    blog_count INT,
    service_count INT,
    store_count INT,
    wiki_count INT,
    work_count INT
);


CREATE TABLE tags_items (
    id SERIAL PRIMARY KEY,
    tag_id INT NOT NULL,
    service_id INT,
    store_id INT,
    blog_id INT,
    wiki_id INT,
    work_id INT,
    tag_created TIMESTAMP NOT NULL
);
