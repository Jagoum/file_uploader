-- Add migration script here
create table IF NOT EXISTS files (
    id SERIAL PRIMARY KEY ,
    file_name varchar(255) not null,
    compressed_file varchar(255),
    created_at timestamptz not null default now()
);