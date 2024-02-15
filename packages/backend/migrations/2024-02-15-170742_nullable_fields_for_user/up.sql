-- Your SQL goes here


ALTER TABLE "users" ALTER COLUMN "display_name" DROP NOT NULL;
ALTER TABLE "users" ALTER COLUMN "dietary_requirements" DROP NOT NULL;
ALTER TABLE "users" ALTER COLUMN "accessibility_requirements" DROP NOT NULL;

