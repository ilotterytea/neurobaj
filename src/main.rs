#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use twitch_irc::{
    login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

mod chains;
mod routes;

static CHAINS: Lazy<Mutex<chains::ChainManager>> =
    Lazy::new(|| Mutex::new(chains::ChainManager::new()));

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(ClientConfig::default());

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });

    client.join("ilotterytea".to_owned()).unwrap();

    rocket::ignite()
        .mount("/api/v1", routes![routes::gen_text])
        .launch();

    join_handle.await.unwrap();
}
