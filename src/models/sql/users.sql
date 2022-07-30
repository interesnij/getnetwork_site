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

-- tags -------
---------------
---------------
CREATE TABLE tags (
    id            SERIAL PRIMARY KEY,
    name          VARCHAR(100) NOT NULL,
    position      SMALLINT NOT NULL,
    count         SMALLINT NOT NULL,
    blog_count    SMALLINT NOT NULL,
    service_count SMALLINT NOT NULL,
    store_count   SMALLINT NOT NULL,
    wiki_count    SMALLINT NOT NULL,
    work_count    SMALLINT NOT NULL,
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
