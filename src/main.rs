#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;

use chains::scan_text;
use diesel::{Connection, QueryDsl, RunQueryDsl, SqliteConnection};
use once_cell::sync::Lazy;
use std::{env, fs::File, path::Path, sync::Mutex, time::Instant};
use tokio_js_set_interval::set_interval;
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
                _ => {}
            }
        }
    });

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
