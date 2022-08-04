
CREATE TABLE stat_mainpages (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);

CREATE TABLE stat_blog_categories (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_service_categories (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_store_categories (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_wiki_categories (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_work_categories (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);

CREATE TABLE stat_tags (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_abouts (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_contacts (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_teams (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_partnerships (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_logins (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_logouts (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_signups (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_infos (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);
CREATE TABLE stat_helps (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);

CREATE TABLE stat_profils (
    id   SERIAL PRIMARY KEY,
    view INT NOT NULL
);


ALTER TABLE blog_categories ADD COLUMN view
INT NOT NULL;
ALTER TABLE service_categories ADD COLUMN view
INT NOT NULL;
ALTER TABLE store_categories ADD COLUMN view
INT NOT NULL;
ALTER TABLE wiki_categories ADD COLUMN view
INT NOT NULL;
ALTER TABLE work_categories ADD COLUMN view
INT NOT NULL;
