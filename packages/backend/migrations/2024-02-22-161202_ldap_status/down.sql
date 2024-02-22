-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "ldap_check_status";
ALTER TABLE "signup_request" DROP COLUMN "ldap_check_status";
