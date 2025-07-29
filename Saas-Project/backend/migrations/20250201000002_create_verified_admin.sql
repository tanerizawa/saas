-- Create verified admin account
-- This migration creates an admin account with email already verified

-- First, check if admin account already exists and delete it
DELETE FROM users WHERE email = 'admin@saas-umkm.local';

-- Insert verified admin account
-- Password hash for "AdminPass123!" using Argon2
INSERT INTO users (
    id,
    email,
    password_hash,
    full_name,
    role,
    status,
    email_verified,
    created_at,
    updated_at
) VALUES (
    uuid_generate_v4(),
    'admin@saas-umkm.local',
    '$argon2id$v=19$m=19456,t=2,p=1$8Nq1pR2qeJCOlIHONPuNJw$lPSghp8ZnrUADjJ3sGD/Yz/K7IHNgJwJhNa3i8e+7tE', -- AdminPass123!
    'System Administrator',
    'user',
    'active',
    TRUE,
    NOW(),
    NOW()
);
