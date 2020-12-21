create table Project (
  id Uuid primary key,
  name varchar not null,
  creator_id Uuid not null,
  creation_data date not null,
  last_update date not null,
  definition varchar(150),
  foreign key (creator_id) references Creator(id),
  constraint uniqueNameForClient unique (name, creator_id)
);