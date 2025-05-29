// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    circle_members (id) {
        id -> Int4,
        circle_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    circles (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        owner_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    shards (id) {
        id -> Int4,
        shard_category -> Int4,
        #[max_length = 128]
        title -> Varchar,
        owner_id -> Int4,
        visibility -> Int4,
        parent_shard -> Nullable<Int4>,
        #[max_length = 128]
        genre -> Nullable<Varchar>,
        shard -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    templates (id) {
        id -> Int4,
        #[max_length = 128]
        title -> Varchar,
        owner_id -> Int4,
        #[max_length = 16]
        version -> Varchar,
        visibility -> Int4,
        template -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_friends (id) {
        id -> Int4,
        user_id -> Int4,
        friend_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 64]
        first_name -> Nullable<Varchar>,
        #[max_length = 64]
        last_name -> Nullable<Varchar>,
        #[max_length = 128]
        email -> Nullable<Varchar>,
        #[max_length = 15]
        phone -> Nullable<Bpchar>,
        bio -> Nullable<Text>,
        #[max_length = 128]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(circle_members -> circles (circle_id));
diesel::joinable!(circle_members -> users (user_id));
diesel::joinable!(circles -> users (owner_id));
diesel::joinable!(shards -> categories (shard_category));
diesel::joinable!(shards -> users (owner_id));
diesel::joinable!(templates -> users (owner_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    circle_members,
    circles,
    roles,
    shards,
    templates,
    user_friends,
    user_roles,
    users,
);
