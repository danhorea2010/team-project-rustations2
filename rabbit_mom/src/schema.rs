// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        task_started -> Bool,
        task_finished -> Bool,
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
    tasks,
    todos,
);
