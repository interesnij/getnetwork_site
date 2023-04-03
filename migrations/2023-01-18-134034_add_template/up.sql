-- Your SQL goes here

ALTER TABLE cookie_stats ADD COLUMN template
VARCHAR(100) NOT NULL DEFAULT 'rhythm'; 
