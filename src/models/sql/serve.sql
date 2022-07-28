-- serve -------
---------------
---------------
-- это технические категории опций (например, большой магазин или моб приложение ресторана)
CREATE TABLE tech_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    position    INT NOT NULL,
    count       INT NOT NULL,
    user_id     INT NOT NULL
);

-- это категория опции (например, rust, python, react native)
CREATE TABLE serve_categories (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    description     VARCHAR(1000),
    cat_name        VARCHAR(100) NOT NULL,
    tech_categories INT NOT NULL,
    position        INT NOT NULL,
    count           INT NOT NULL,
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
    position         INT NOT NULL,
    serve_categories INT NOT NULL,
    price            INT NOT NULL,
    man_hours        INT NOT NULL,
    is_default       BOOLEAN NOT NULL, -- опция по умолчанию, т.е. без которой работа невозможна (например, админка)
    user_id          INT NOT NULL,

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
