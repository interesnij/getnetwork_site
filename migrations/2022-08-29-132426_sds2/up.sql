-- Your SQL goes here
ALTER TABLE orders ADD COLUMN price
INT NOT NULL;

ALTER TABLE orders ADD COLUMN price_acc
INT;
