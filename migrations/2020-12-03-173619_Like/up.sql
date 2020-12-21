create table Liked (
    id uuid primary key,
    comment_id uuid not null,
    user_id uuid not null,
    foreign key (comment_id) references Comment(id),
    foreign key (user_id) references Creator(id),
    constraint UniqueLike unique (comment_id, user_id)
)