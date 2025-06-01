-- This file should undo anything in `up.sql`

ALTER TABLE "Website" 
ALTER COLUMN "timeAdded" DROP DEFAULT;