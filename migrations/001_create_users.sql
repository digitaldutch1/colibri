


-- 1. Table: user (Admin/Staff accounts)
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20),
    password_hash TEXT NOT NULL,
    role VARCHAR(10) NOT NULL DEFAULT 'staff' CHECK (role IN ('admin', 'staff')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 2. Table: customer (Public guests)
CREATE TABLE IF NOT EXISTS customer (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20),
    address VARCHAR(255),
    postal_code VARCHAR(20),
    city VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 3. Table: accommodation (Types Chalet, Tent and Pitch)
CREATE TABLE IF NOT EXISTS accommodation (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    total_units INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 4. Table: unit (Specific spots/codes C, T and P)
CREATE TABLE IF NOT EXISTS unit (
    id SERIAL PRIMARY KEY,
    accommodation_id INT NOT NULL REFERENCES accommodation(id) ON DELETE CASCADE,
    unit_code VARCHAR(10) NOT NULL, -- (C1, C2, C3, T1, T2, T3, P1, P2 and P3)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 5. Table: booking
CREATE TABLE IF NOT EXISTS booking (
    id SERIAL PRIMARY KEY,
    customer_id INT NOT NULL REFERENCES customer(id) ON DELETE CASCADE,
    accommodation_id INT NOT NULL REFERENCES accommodation(id),
    unit_id INT NOT NULL REFERENCES unit(id),
    check_in_date DATE NOT NULL,
    check_out_date DATE NOT NULL,
    total_price DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- (pending, confirmed, cancelled, expired)
    payment_token TEXT,
    cancel_token TEXT,
    invoice_number TEXT,
    source TEXT, -- (colibri, blookers)
    external_reference VARCHAR(100),
    created_by_user_id INT REFERENCES "user"(id) ON DELETE SET NULL, -- NULL if via public website
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMP NULL
);


-- Add user to user table
INSERT INTO "user" (first_name, last_name, email, password_hash, role)
VALUES ('Dennis', 'Hettinga', 'dennis_hettinga@live.nl', '1111', 'admin');

-- Add units to accommodation table
INSERT INTO accommodation (name, total_units) 
VALUES 
('Chalet', 3), 
('Tent', 3), 
('Pitch', 3);

-- Add unit id's to unit table and connect them to accommodation table
INSERT INTO unit (accommodation_id, unit_code) VALUES 
(1, 'C1'), (1, 'C2'), (1, 'C3'),
(2, 'T1'), (2, 'T2'), (2, 'T3'),
(3, 'P1'), (3, 'P2'), (3, 'P3'); 

-- Check table
SELECT * FROM "booking";
SELECT * FROM "customer";
SELECT * FROM "user";
SELECT * FROM "accommodation";

-- Empty table
DELETE FROM booking;
DELETE FROM customer;
DELETE FROM "user";

-- Resette id's
TRUNCATE TABLE customer RESTART IDENTITY CASCADE;
TRUNCATE booking, customer RESTART IDENTITY CASCADE;
TRUNCATE booking, customer, "user" RESTART IDENTITY CASCADE;


UPDATE booking
SET created_at = NOW() - INTERVAL '8 days'
WHERE status = 'pending';