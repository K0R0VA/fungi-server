create table Creator (
    id Uuid primary key,
    name varchar not null unique,
    email varchar not null unique,
    bio varchar(150),
    password varchar not null,
    hasAvatar boolean not null
);