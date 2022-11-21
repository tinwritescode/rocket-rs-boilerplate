-- email_verify_add can be null, sqlite
ALTER TABLE users ADD COLUMN email_verified_at DATETIME NULL;