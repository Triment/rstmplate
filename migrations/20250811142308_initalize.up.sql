-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists "users" (
    id uuid primary default uuid_generate_v4(),
    username text unique not null,
    password_hash text not null,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

create index on "users" (created_at desc);