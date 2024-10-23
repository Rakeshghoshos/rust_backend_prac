-- Add up migration script here
create type user_role as enum ('admin', 'user');
create extension if not exists "uuid-ossp";
create table users (
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null,
    email text not null unique,
    password text not null,
    role user_role not null default 'user',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
)