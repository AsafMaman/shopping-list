-- Your current migration could be updated to:
CREATE TABLE users (
    id TEXT PRIMARY KEY,                    -- UUID as text
    first_name VARCHAR(100),                -- Reasonable limit
    last_name VARCHAR(100),                 -- Reasonable limit  
    email VARCHAR(255),                     -- Standard email limit
    password_hash VARCHAR(255),             -- Hash length is known
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
