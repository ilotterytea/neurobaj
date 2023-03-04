use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::models::{Chain, NewChain, NewSignature, Signature};

pub fn tokenize_text(text: &str) -> Vec<(String, String)> {
    let mut chains: Vec<(String, String)> = Vec::new();

    let s = text.split(' ').collect::<Vec<&str>>();

    let mut prev_word = "\\x02";

    for w in s {
        chains.push((prev_word.to_string(), w.to_string()));
        prev_word = w;
    }

    chains.push((prev_word.to_string(), "\\x03".to_string()));

    chains
}
pub fn scan_text(
    conn: &mut SqliteConnection,
    user_id: &str,
    channel_id: &str,
    message_id: &str,
    text: &str,
) {
    use crate::schema::chains::dsl::*;

    let tokens = tokenize_text(text);
    let signature = get_signature(conn, user_id, channel_id);

    for token in tokens {
        let chain = chains
            .filter(from_word.eq(token.0.as_str()))
            .first::<Chain>(conn);

        if chain.is_err() {
            insert_into(chains)
                .values(vec![NewChain {
                    from_word: token.0.as_str(),
                    to_word: token.1.as_str(),
                    to_word_signature_id: signature.id.clone(),
                    from_word_signature_id: signature.id.clone(),
                    msg_id: message_id,
                }])
                .execute(conn)
                .expect("Cannot insert the values!");
        } else {
            update(chains.filter(from_word.eq(token.0.as_str())))
                .set((
                    to_word.eq(token.1.as_str()),
                    to_word_signature_id.eq(signature.id.clone()),
                ))
                .execute(conn)
                .expect("Cannot update the values!");
        }
    }
}
pub fn get_signature(conn: &mut SqliteConnection, user_id: &str, target_id: &str) -> Signature {
    use crate::schema::signatures::dsl::*;

    let mut signature = signatures
        .filter(sender_id.eq(user_id.parse::<i32>().unwrap()))
        .filter(channel_id.eq(target_id.parse::<i32>().unwrap()))
        .first::<Signature>(conn);

    if signature.is_err() {
        insert_into(signatures)
            .values(vec![NewSignature {
                channel_id: target_id.parse::<i32>().unwrap(),
                sender_id: user_id.parse::<i32>().unwrap(),
            }])
            .execute(conn)
            .expect("Cannot insert the values!");

        signature = signatures
            .filter(sender_id.eq(user_id.parse::<i32>().unwrap()))
            .filter(channel_id.eq(target_id.parse::<i32>().unwrap()))
            .first::<Signature>(conn);
    }

    signature.unwrap()
}

pub fn generate_text(conn: &mut SqliteConnection, initial_text: &str) -> String {
    use crate::schema::chains::dsl::*;

    let s = initial_text.split(' ').collect::<Vec<&str>>();
    let mut message = String::new();

    for w in s {
        let first_chain = chains.filter(from_word.eq(w)).first::<Chain>(conn);

        if first_chain.is_err() {
            continue;
        }

        let _fc = first_chain.unwrap();
        let mut next_chain: Option<Chain> = None;

        loop {
            if next_chain.is_none() {
                message.push_str(_fc.from_word.as_str());
                message.push(' ');

                let chain = chains
                    .filter(from_word.eq(&_fc.to_word))
                    .first::<Chain>(conn);

                if chain.is_err() {
                    break;
                }

                next_chain = Some(chain.unwrap());
            } else {
                let _nc = next_chain.unwrap();
                message.push_str(_nc.from_word.as_str());
                message.push(' ');

                let chain = chains
                    .filter(from_word.eq(&_nc.to_word))
                    .first::<Chain>(conn);

                if chain.is_err() {
                    break;
                }

                next_chain = Some(chain.unwrap());
            }
        }
    }

    if message.len() == 0 {
        message.push_str("...");
    }

    message
}
