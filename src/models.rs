use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Chain {
    pub id: i32,
    pub from_word: String,
    pub to_word: String,
    pub from_word_signature_id: i32,
    pub to_word_signature_id: i32,
    pub msg_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = chains)]
pub struct NewChain<'a> {
    pub from_word: &'a str,
    pub to_word: &'a str,
    pub from_word_signature_id: i32,
    pub to_word_signature_id: i32,
    pub msg_id: &'a str,
}

#[derive(Queryable)]
pub struct Channel {
    pub id: i32,
    pub alias_id: String,
    pub platform_id: i32,
    pub ignore_phrases: Option<String>,
    pub created_timestamp: i32,
    pub last_timestamp: i32,
    pub enabled: i32,
}

#[derive(Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel<'a> {
    pub alias_id: &'a str,
    pub platform_id: i32,
    pub created_timestamp: i32,
    pub last_timestamp: i32,
}

#[derive(Queryable)]
pub struct Signature {
    pub id: i32,
    pub channel_id: i32,
    pub sender_id: i32,
    pub is_ignored: i32,
}

#[derive(Insertable)]
#[diesel(table_name = signatures)]
pub struct NewSignature {
    pub channel_id: i32,
    pub sender_id: i32,
}
