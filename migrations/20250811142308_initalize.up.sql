-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists "users" (
    id uuid primary key default uuid_generate_v4(),
    username varchar(255) unique not null,
    password_hash text not null,
    created_at TIMESTAMPTZ NOT NULL default now(),
    updated_at TIMESTAMPTZ NOT NULL default now()
);

create index on "users" (created_at desc);