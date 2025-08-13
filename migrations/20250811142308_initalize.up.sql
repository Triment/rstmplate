-- Add up migration script here
create extension if not exists "pgcrypto";

create table if not exists "users" (
    id uuid primary key default gen_random_uuid(), -- 性能比 uuid_generate_v4() 更好
    username varchar(255) unique not null,
    password_hash text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index on "users" (created_at desc);

create or replace function update_updated_at()
returns trigger as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;

create trigger set_updated_at
before update on "users"
for each row
execute function update_updated_at();