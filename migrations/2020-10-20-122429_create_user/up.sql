create table Client (
    id Uuid primary key,
    name varchar not null,
    email varchar not null,
    password varchar not null
);