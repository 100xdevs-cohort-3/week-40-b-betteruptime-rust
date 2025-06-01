-- This file should undo anything in `up.sql`
ALTER TABLE "website"
ALTER COLUMN "time_added"
DROP DEFAULT;
