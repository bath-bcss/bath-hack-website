// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Uuid,
        #[max_length = 8]
        join_code -> Nullable<Bpchar>,
    }
}

diesel::table! {
    signup_request (id) {
        id -> Uuid,
        bath_username -> Text,
        created_at -> Timestamp,
        expires -> Timestamp,
        secret_hash -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        display_name -> Text,
        bath_username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        dietary_requirements -> Text,
        accessibility_requirements -> Text,
        group_id -> Nullable<Uuid>,
    }
}

diesel::joinable!(users -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    signup_request,
    users,
);
