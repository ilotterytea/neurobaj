#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use tokio_js_set_interval::set_interval;
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

    client.join("ilotterytea".to_owned()).unwrap();

    rocket::ignite()
        .mount("/api/v1", routes![routes::gen_text])
        .launch();

    set_interval!(|| { CHAINS.lock().unwrap().save("./chains.json") }, 90000);
    join_handle.await.unwrap();
}
