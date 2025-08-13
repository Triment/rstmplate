-- Add down migration script here
drop table if exists "users";
drop trigger if exists set_updated_at on "users";
drop function if exists update_updated_at();
drop extension if exists "pgcrypto";