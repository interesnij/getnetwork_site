-- Your SQL goes here

ALTER TABLE help_items ADD COLUMN position
SMALLINT NOT NULL DEFAULT 0;
