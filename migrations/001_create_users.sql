


-- 1. Table: user (Admin/Staff accounts)
CREATE TABLE IF NOT EXISTS "user" (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(20) NOT NULL, -- (admin, staff)
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
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMP NULL
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
    status VARCHAR(20) DEFAULT 'pending', -- (pending, confirmed, cancelled)
    payment_token TEXT,
    cancel_token TEXT,
    invoice_number TEXT,
    source TEXT, -- (colibri, blookers)
    external_reference VARCHAR(100),
    created_by_user_id INT REFERENCES "user"(id) ON DELETE SET NULL, -- NULL if via public website
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);