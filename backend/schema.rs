// @generated automatically by Diesel CLI.

diesel::table! {
    role_permissions (role, permission) {
        role -> Text,
        permission -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    todos (id) {
        id -> Integer,
        text -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_permissions (user_id, permission) {
        user_id -> Integer,
        permission -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (user_id, role) {
        user_id -> Integer,
        role -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Integer,
        user_id -> Integer,
        refresh_token -> Text,
        device -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        hash_password -> Text,
        activated -> Bool,
        created_at -> Timestamp,
    }
}

diesel::joinable!(user_permissions -> users (user_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    role_permissions,
    todos,
    user_permissions,
    user_roles,
    user_sessions,
    users,
);
