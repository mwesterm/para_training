// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        create_date -> Timestamp,
    }
}
