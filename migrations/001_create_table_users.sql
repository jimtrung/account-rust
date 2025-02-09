CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(), 
    deleted_at TIMESTAMP DEFAULT NOW() 
);

INSERT INTO users (username, email, password) VALUES
('trung', 'trung@example.com', 'hashed_password_1'),
('hai', 'hai@example.com', 'hashed_password_2'),
('john_doe', 'john@example.com', 'hashed_password_3'),
('alice', 'alice@example.com', 'hashed_password_4'),
('bob', 'bob@example.com', 'hashed_password_5'),
('charlie', 'charlie@example.com', 'hashed_password_6'),
('david', 'david@example.com', 'hashed_password_7'),
('eva', 'eva@example.com', 'hashed_password_8'),
('frank', 'frank@example.com', 'hashed_password_9'),
('grace', 'grace@example.com', 'hashed_password_10');

DROP TABLE users;
DELETE FROM users;
SELECT * FROM users;
