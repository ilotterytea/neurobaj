// @generated automatically by Diesel CLI.

diesel::table! {
    chains (id) {
        id -> Integer,
        from_word -> Text,
        to_word -> Text,
        from_word_signature_id -> Integer,
        to_word_signature_id -> Integer,
        msg_id -> Text,
    }
}

diesel::table! {
    channels (id) {
        id -> Integer,
        alias_id -> Integer,
        platform_id -> Integer,
        channel_name -> Integer,
        ignore_phrases -> Nullable<Text>,
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
