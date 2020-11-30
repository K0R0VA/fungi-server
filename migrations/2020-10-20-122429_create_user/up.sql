create table Client (
    id Uuid primary key,
    name varchar not null unique,
    email varchar not null unique,
    password varchar not null
);