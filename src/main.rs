#![feature(proc_macro_hygiene, decl_macro, once_cell)]

use once_cell::sync::Lazy;
use std::sync::Mutex;

mod chains;

static CHAINS: Lazy<Mutex<chains::ChainManager>> =
    Lazy::new(|| Mutex::new(chains::ChainManager::new()));

fn main() {
    println!("Hello, world!");
}
