-- Add migration script here

create table files (
    id SERIAL PRIMARY KEY DEFAULT,
    file_name varchar(255) not null,
    compressed_file varchar(255),
    created_at timestamptz not null default now()
);