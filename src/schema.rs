// @generated automatically by Diesel CLI.

diesel::table! {
    chains (id) {
        id -> Integer,
        from_word -> Text,
        to_word -> Text,
        from_word_signature_id -> Integer,
        to_word_signature_id -> Integer,
        from_word_msg_id -> Text,
        to_word_msg_id -> Text,
    }
}

diesel::table! {
    channels (id) {
        id -> Integer,
        alias_id -> Text,
        platform_id -> Integer,
        ignore_phrases -> Nullable<Text>,
        created_timestamp -> Integer,
        last_timestamp -> Integer,
        enabled -> Integer,
    }
}

diesel::table! {
    signatures (id) {
        id -> Integer,
        channel_id -> Integer,
        sender_id -> Integer,
        is_ignored -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chains,
    channels,
    signatures,
);
