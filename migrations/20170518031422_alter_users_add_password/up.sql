-- Your SQL goes here
ALTER TABLE user
ADD COLUMN password VARCHAR(60) NOT NULL AFTER email;

