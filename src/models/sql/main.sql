
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
    view          INT NOT NULL,

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
    view        INT NOT NULL,

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


-- serve -------
---------------
---------------
-- это технические категории опций (например, большой магазин или моб приложение ресторана)
CREATE TABLE tech_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(3000),
    position    SMALLINT NOT NULL,
    count       SMALLINT NOT NULL,
    level       SMALLINT NOT NULL,
    user_id     INT NOT NULL
);

-- это категория опции (например, rust, python, react native)
CREATE TABLE serve_categories (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    description     VARCHAR(3000),
    cat_name        VARCHAR(100) NOT NULL,
    tech_categories INT NOT NULL,
    position        SMALLINT NOT NULL,
    count           SMALLINT NOT NULL,
    default_price   INT NOT NULL, -- сумма всех опуий по умолчанию.
    user_id         INT NOT NULL,

    CONSTRAINT fk_tech_category
        FOREIGN KEY(tech_categories)
            REFERENCES tech_categories(id)
);

-- это опции (например, продвинутая админка)
CREATE TABLE serve (
    id               SERIAL PRIMARY KEY,
    name             VARCHAR(100) NOT NULL,
    cat_name         VARCHAR(100) NOT NULL,
    description      VARCHAR(10000),
    position         SMALLINT NOT NULL,
    serve_categories INT NOT NULL,
    price            INT NOT NULL,
    man_hours        SMALLINT NOT NULL,
    is_default       BOOLEAN NOT NULL, -- опция по умолчанию, т.е. без которой работа невозможна (например, админка)
    user_id          INT NOT NULL,
    tech_cat_id      INT NOT NULL,
    types            VARCHAR(100),     -- класс опции для организации выбора между несколькими опциями

    CONSTRAINT fk_serve_category
        FOREIGN KEY(serve_categories)
            REFERENCES serve_categories(id),
    CONSTRAINT fk_serve_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- связь опции с объетками сервисов, работ, товаров
CREATE TABLE serve_items (
    id         SERIAL PRIMARY KEY,
    serve_id   INT NOT NULL,
    service_id INT NOT NULL, -- нужно для списка id опций в счетчик выбранных опций. Поместим туда опции тех категорий по умолчанию, активных
    store_id   INT NOT NULL,
    work_id    INT NOT NULL
);

-- это те tech_categories, которые привязываются к объеткам.
-- бывают открытые (активные) и дополнительные.
CREATE TABLE tech_categories_items (
    id          SERIAL PRIMARY KEY,
    category_id INT NOT NULL,     -- тех. категория (например, создание среднего магазина)
    service_id  INT NOT NULL,     -- услуга
    store_id    INT NOT NULL,     -- товар
    work_id     INT NOT NULL,     -- работа
    types       SMALLINT NOT NULL -- тип: 1 - активно, 2 - неактивно
);



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

-- stores -------
---------------
---------------
CREATE TABLE store_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    position    SMALLINT NOT NULL,
    image       VARCHAR(500),
    count       SMALLINT NOT NULL
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
    position     SMALLINT NOT NULL,

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


-- works -------
---------------
---------------
CREATE TABLE work_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    position    SMALLINT NOT NULL,
    image       VARCHAR(500),
    count       SMALLINT NOT NULL
);

CREATE TABLE works (
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
