#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use std::{fs::File, path::Path, sync::Mutex};
use tokio_js_set_interval::set_interval;
use twitch_irc::{
    login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

mod chains;
mod routes;

static CHAINS: Lazy<Mutex<chains::ChainManager>> =
    Lazy::new(|| Mutex::new(chains::ChainManager::new()));

static CHANNELS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut _vec: Vec<String> = Vec::new();

    if !Path::new("./channels.json").exists() {
        return _vec;
    }

    let file = File::open("./channels.json").unwrap();
    let mut _v: Vec<String> =
        serde_json::from_reader(file).expect("JSON file with channels is not well formatted!");

    _vec.append(&mut _v);

    _vec
});

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    CHAINS.lock().unwrap().load("./chains.json");

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(ClientConfig::default());

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);

            match message {
                twitch_irc::message::ServerMessage::Privmsg(msg) => {
                    CHAINS.lock().unwrap().scan_text(
                        &msg.message_text,
                        Some(chains::ChainSignature {
                            msg_id: Some(msg.message_id),
                            author_id: Some(msg.sender.id),
                            channel_id: Some(msg.channel_id),
                        }),
                    );
                }
                _ => {}
            }
        }
    });

    for name in &CHANNELS.to_vec() {
        client.join(name.to_owned()).unwrap();
    }

    rocket::ignite()
        .mount("/api/v1", routes![routes::gen_text])
        .launch();

    set_interval!(
        || {
            CHAINS.lock().unwrap().save("./chains.json");

            std::fs::write(
                "./channels.json",
                serde_json::to_string_pretty(&CHANNELS.to_vec()).unwrap(),
            )
            .unwrap();
            println!("SAVED CHANNELS!");
        },
        90000
    );
    join_handle.await.unwrap();
}
