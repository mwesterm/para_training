// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        create_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        lastname -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    app_users,
    students,
);
