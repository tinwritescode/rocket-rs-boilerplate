// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        expired_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        email_verified_at -> Nullable<Timestamp>,
        role -> Nullable<Varchar>,
    }
}

diesel::joinable!(posts -> users (user_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(posts, tokens, users,);
