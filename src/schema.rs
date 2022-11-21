// @generated automatically by Diesel CLI.

diesel::table! {
    heroes (id) {
        id -> Integer,
        fantasy_name -> Text,
        real_name -> Nullable<Text>,
        spotted_photo -> Text,
        strength_level -> Integer,
    }
}

diesel::table! {
    tokens (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        email_verified_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(heroes, tokens, users,);
