#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;

use chains::scan_text;
use diesel::{delete, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use once_cell::sync::Lazy;
use std::{env, sync::Mutex, time::Instant};
use twitch_api::{
    twitch_oauth2::{AppAccessToken, Scope},
    types::UserIdRef,
    TwitchClient,
};
use twitch_irc::{
    login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

mod chains;
mod models;
mod routes;
mod schema;

static ELAPSED_TIME: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Hello, world!");

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(ClientConfig::default());

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);

            match message {
                twitch_irc::message::ServerMessage::Privmsg(msg) => scan_text(
                    &mut establish_connection(),
                    msg.sender.id.as_str(),
                    msg.channel_id.as_str(),
                    msg.message_id.as_str(),
                    msg.message_text.as_str(),
                ),
                twitch_irc::message::ServerMessage::ClearMsg(msg) => {
                    delete(crate::schema::chains::dsl::chains.filter(
                        crate::schema::chains::dsl::from_word_msg_id.eq(msg.message_id.clone()),
                    ))
                    .execute(&mut establish_connection())
                    .expect("Cannot delete the values!");
                    delete(
                        crate::schema::chains::dsl::chains
                            .filter(crate::schema::chains::dsl::to_word_msg_id.eq(msg.message_id)),
                    )
                    .execute(&mut establish_connection())
                    .expect("Cannot delete the values!");
                }
                twitch_irc::message::ServerMessage::ClearChat(msg) => match msg.action {
                    twitch_irc::message::ClearChatAction::UserBanned {
                        user_login: _,
                        user_id,
                    } => {
                        let conn = &mut establish_connection();
                        let signature = crate::chains::get_signature(
                            conn,
                            user_id.as_str(),
                            msg.channel_id.as_str(),
                        );
                        delete(crate::schema::chains::dsl::chains.filter(
                            crate::schema::chains::dsl::from_word_signature_id.eq(signature.id),
                        ))
                        .execute(conn)
                        .expect("Cannot delete the values!");
                        delete(crate::schema::chains::dsl::chains.filter(
                            crate::schema::chains::dsl::to_word_signature_id.eq(signature.id),
                        ))
                        .execute(conn)
                        .expect("Cannot delete the values!");
                    }
                    twitch_irc::message::ClearChatAction::UserTimedOut {
                        user_login: _,
                        user_id,
                        timeout_length: _,
                    } => {
                        let conn = &mut establish_connection();
                        let signature = crate::chains::get_signature(
                            conn,
                            user_id.as_str(),
                            msg.channel_id.as_str(),
                        );
                        delete(crate::schema::chains::dsl::chains.filter(
                            crate::schema::chains::dsl::from_word_signature_id.eq(signature.id),
                        ))
                        .execute(conn)
                        .expect("Cannot delete the values!");
                        delete(crate::schema::chains::dsl::chains.filter(
                            crate::schema::chains::dsl::to_word_signature_id.eq(signature.id),
                        ))
                        .execute(conn)
                        .expect("Cannot delete the values!");
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    });

    let api_client: TwitchClient<reqwest::Client> = TwitchClient::default();
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set!");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set!");
    let token = AppAccessToken::get_app_access_token(
        &api_client,
        client_id.into(),
        client_secret.into(),
        Scope::all(),
    )
    .await
    .unwrap();

    let db_channels = crate::schema::channels::dsl::channels
        .filter(crate::schema::channels::dsl::enabled.eq(1))
        .filter(crate::schema::channels::dsl::platform_id.eq(0))
        .select(crate::schema::channels::dsl::alias_id)
        .load::<String>(&mut establish_connection())
        .expect("");

    for id in db_channels {
        let _id: &UserIdRef = id.as_str().into();
        let c = &api_client
            .helix
            .get_user_from_id(_id, &token)
            .await
            .unwrap();

        if c.is_some() {
            let _c = c.clone().unwrap();
            client.join(_c.login.to_string()).unwrap();
        }
    }

    rocket::ignite()
        .mount("/api/v1", routes![routes::gen_text, routes::status])
        .launch();

    join_handle.await.unwrap();
}

pub fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
