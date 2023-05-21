// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    agendas (id) {
        id -> Int4,
        title -> Varchar,
        deadline -> Timestamp,
    }
}

diesel::table! {
    todo (id) {
        id -> Int8,
        description -> Nullable<Varchar>,
        title -> Varchar,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        visibility -> Int2,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    agendas,
    todo,
    todos,
);
