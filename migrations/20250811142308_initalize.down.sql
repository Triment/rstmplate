-- Add down migration script here
drop table if exists "users";
drop extension if exists "uuid-ossp";