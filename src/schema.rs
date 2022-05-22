table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    locations (id) {
        id -> Int4,
        loc -> Geography,
        is_active -> Bool,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        pub_date -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    locations,
    posts,
    spatial_ref_sys,
);
