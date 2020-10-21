table! {
    animation (id) {
        id -> Uuid,
        mongo_id -> Int4,
        name -> Varchar,
        like_count -> Int4,
        creation_data -> Date,
        last_update -> Date,
        creator_id -> Uuid,
    }
}

table! {
    client (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    project (id) {
        id -> Uuid,
        name -> Varchar,
        mongo_id -> Int4,
        client_id -> Uuid,
        creation_data -> Date,
        last_update -> Date,
    }
}

joinable!(animation -> client (creator_id));
joinable!(project -> client (client_id));

allow_tables_to_appear_in_same_query!(
    animation,
    client,
    project,
);
