table! {
    comment (id) {
        id -> Uuid,
        user_id -> Uuid,
        plugin_id -> Uuid,
        like_count -> Int4,
        content -> Varchar,
    }
}

table! {
    creator (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        bio -> Nullable<Varchar>,
        password -> Varchar,
        hasavatar -> Bool,
    }
}

table! {
    import (id) {
        id -> Uuid,
        project_id -> Uuid,
        plugin_id -> Uuid,
    }
}

table! {
    liked (id) {
        id -> Uuid,
        comment_id -> Uuid,
        user_id -> Uuid,
    }
}

table! {
    plugin (id) {
        id -> Uuid,
        name -> Varchar,
        import_count -> Int4,
        creation_data -> Date,
        last_update -> Date,
        definition -> Nullable<Varchar>,
        public -> Bool,
        weight -> Float8,
        creator_id -> Uuid,
    }
}

table! {
    project (id) {
        id -> Uuid,
        name -> Varchar,
        creator_id -> Uuid,
        creation_data -> Date,
        last_update -> Date,
        definition -> Nullable<Varchar>,
    }
}

joinable!(comment -> creator (user_id));
joinable!(comment -> plugin (plugin_id));
joinable!(import -> plugin (plugin_id));
joinable!(import -> project (project_id));
joinable!(liked -> comment (comment_id));
joinable!(liked -> creator (user_id));
joinable!(plugin -> creator (creator_id));
joinable!(project -> creator (creator_id));

allow_tables_to_appear_in_same_query!(
    comment,
    creator,
    import,
    liked,
    plugin,
    project,
);
