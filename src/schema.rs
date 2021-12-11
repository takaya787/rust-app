table! {
    active_storage_attachments (id) {
        id -> Int8,
        name -> Varchar,
        record_type -> Varchar,
        record_id -> Int8,
        blob_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    active_storage_blobs (id) {
        id -> Int8,
        key -> Varchar,
        filename -> Varchar,
        content_type -> Nullable<Varchar>,
        metadata -> Nullable<Text>,
        service_name -> Varchar,
        byte_size -> Int8,
        checksum -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    active_storage_variant_records (id) {
        id -> Int8,
        blob_id -> Int8,
        variation_digest -> Varchar,
    }
}

table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    likes (id) {
        id -> Int8,
        user_id -> Int8,
        micropost_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    microposts (id) {
        id -> Int8,
        content -> Nullable<Text>,
        user_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    relationships (id) {
        id -> Int8,
        follower_id -> Int8,
        followed_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        password_digest -> Nullable<Varchar>,
        admin -> Bool,
        activation_digest -> Nullable<Varchar>,
        activated -> Nullable<Bool>,
        activated_at -> Nullable<Timestamp>,
        reset_digest -> Nullable<Varchar>,
        reset_sent_at -> Nullable<Timestamp>,
    }
}

joinable!(active_storage_attachments -> active_storage_blobs (blob_id));
joinable!(active_storage_variant_records -> active_storage_blobs (blob_id));
joinable!(likes -> microposts (micropost_id));
joinable!(likes -> users (user_id));
joinable!(microposts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    active_storage_attachments,
    active_storage_blobs,
    active_storage_variant_records,
    ar_internal_metadata,
    likes,
    microposts,
    relationships,
    schema_migrations,
    users,
);
