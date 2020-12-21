create table Comment(
  id uuid primary key,
  user_id uuid not null,
  plugin_id uuid not null,
  like_count int not null default(0),
  content varchar not null,
  foreign key (plugin_id) references Plugin(id),
  foreign key (user_id) references Creator(id)
);