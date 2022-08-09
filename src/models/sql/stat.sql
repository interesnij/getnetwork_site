
CREATE TABLE stat_mainpages (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);

CREATE TABLE stat_blog_categories (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);
CREATE TABLE stat_service_categories (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);
CREATE TABLE stat_store_categories (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);
CREATE TABLE stat_wiki_categories (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);
CREATE TABLE stat_work_categories (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);

CREATE TABLE stat_tags (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);

CREATE TABLE stat_infos (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);
CREATE TABLE stat_helps (
    id      SERIAL PRIMARY KEY,
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);
