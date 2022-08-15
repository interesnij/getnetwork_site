-- Your SQL goes here

ALTER TABLE serve DROP COLUMN types;

ALTER TABLE serve ADD COLUMN serve_id INT NOT NULL;

ALTER TABLE services ADD COLUMN price_acc INT;

ALTER TABLE stores ADD COLUMN price_acc INT;

ALTER TABLE works ADD COLUMN price_acc INT;
