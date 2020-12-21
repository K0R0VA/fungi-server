create table Plugin (
  id uuid primary key,
  name varchar not null,
  import_count int not null default 0,
  creation_data date not null,
  last_update date not null,
  definition varchar(150),
  public bool not null,
  weight float not null,
  creator_id uuid not null,
  foreign key (creator_id) references Creator(id)
);