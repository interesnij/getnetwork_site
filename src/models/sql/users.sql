-- feedback -------
---------------
---------------
CREATE TABLE feedbacks (
    id       SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(200) NOT NULL,
    message  VARCHAR(1000) NOT NULL
);

-- help -------
---------------
---------------
CREATE TABLE help_item_categories (
    id    SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL
);

CREATE TABLE help_items (
    id          SERIAL PRIMARY KEY,
    category_id INT NOT NULL,
    title       VARCHAR(200) NOT NULL,
    content     VARCHAR(1000) NOT NULL
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

CREATE TABLE cookie_users (
    id         SERIAL PRIMARY KEY,
    ip         VARCHAR(100) NOT NULL, -- ip адрес пользователя
    device     SMALLINT NOT NULL,     -- комп - смартфон - планшет
    city_ru    VARCHAR(150),          -- город по русски
    city_en    VARCHAR(150),          -- город по английски
    region_ru  VARCHAR(150),          -- регион по русски
    region_en  VARCHAR(150),          -- регион по английски
    country_ru VARCHAR(150),          -- страна по русски
    country_en VARCHAR(150),          -- страна по английски
    created    TIMESTAMP NOT NULL     -- когда создан пользователь
);
CREATE TABLE cookie_stats (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,          -- связь с пользователем куки
    page       SMALLINT NOT NULL,     -- номер страницы, которая просматривается
    link       VARCHAR(200) NOT NULL, -- ссылка страницы
    title      VARCHAR(200) NOT NULL, -- название страницы
    height     FLOAT NOT NULL,        -- высота просмотра страницы
    speed      SMALLINT NOT NULL,     -- секунды нахождения страницы
    created    TIMESTAMP NOT NULL,    -- когда создана запись

    CONSTRAINT fk_cookie_stat_user
        FOREIGN KEY(user_id)
            REFERENCES cookie_users(id)
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
