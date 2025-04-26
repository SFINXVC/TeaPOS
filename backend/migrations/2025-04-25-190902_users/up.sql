-- Your SQL goes here
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('superadmin', 'admin', 'user', 'employee');
EXCEPTION 
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    fullname VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    whatsapp VARCHAR(15) NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Apply the diesel_manage_updated_at function to automatically update the updated_at timestamp
SELECT diesel_manage_updated_at('users');