-- This file should undo anything in `up.sql`
-- remove column expired at
ALTER TABLE tokens DROP COLUMN expired_at;