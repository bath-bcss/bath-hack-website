-- This file should undo anything in `up.sql`


ALTER TABLE "users" ALTER COLUMN "display_name" SET NOT NULL;
ALTER TABLE "users" ALTER COLUMN "dietary_requirements" SET NOT NULL;
ALTER TABLE "users" ALTER COLUMN "accessibility_requirements" SET NOT NULL;

