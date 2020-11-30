create table Animation (
  id uuid primary key,
  mongo_id int not null unique,
  name varchar not null,
  like_count int not null default 0,
  creation_data date not null,
  last_update date not null,
  creator_id uuid not null,
  foreign key (creator_id) references Client (id)
);