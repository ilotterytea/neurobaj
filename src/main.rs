#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use std::sync::Mutex;

mod chains;
mod routes;

static CHAINS: Lazy<Mutex<chains::ChainManager>> =
    Lazy::new(|| Mutex::new(chains::ChainManager::new()));

fn main() {
    println!("Hello, world!");

    rocket::ignite()
        .mount("/api/v1", routes![routes::gen_text])
        .launch();
}
