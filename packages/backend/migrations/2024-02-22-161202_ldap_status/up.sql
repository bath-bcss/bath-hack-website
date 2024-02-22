-- Your SQL goes here
ALTER TABLE "users" ADD COLUMN "ldap_check_status" smallint not null default 0;
ALTER TABLE "signup_request" ADD COLUMN "ldap_check_status" smallint not null default 0;
