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
        path -> Varchar,
        client_id -> Uuid,
    }
}

joinable!(project -> client (client_id));

allow_tables_to_appear_in_same_query!(
    client,
    project,
);
