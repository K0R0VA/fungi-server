create table Project (
  id Uuid primary key,
  name varchar not null,
  mongo_id int not null,
  client_id Uuid not null,
  creation_data date not null,
  last_update date not null,
  foreign key (client_id) references Client(id)
);