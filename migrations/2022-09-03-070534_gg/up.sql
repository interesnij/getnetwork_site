-- Your SQL goes here

ALTER TABLE help_item_categories ADD COLUMN view
INT NOT NULL DEFAULT 0;
ALTER TABLE help_item_categories ADD COLUMN height
FLOAT NOT NULL DEFAULT 0.0; 
ALTER TABLE help_item_categories ADD COLUMN seconds
INT NOT NULL DEFAULT 0;
ALTER TABLE help_item_categories ADD COLUMN position
INT NOT NULL DEFAULT 0;
