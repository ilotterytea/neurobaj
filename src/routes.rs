use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::{CHAINS, CHANNELS, ELAPSED_TIME};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub status: usize,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub total_chains: usize,
    pub joined_channels: Vec<String>,
    pub uptime_ms: u128,
}

#[get("/gen?<message>")]
pub fn gen_text(message: String) -> String {
    CHAINS.lock().unwrap().generate_text(&message)
}

#[get("/status")]
pub fn status() -> Json<Response<Status>> {
    Json(Response {
        status: 200,
        data: Status {
            total_chains: CHAINS.lock().unwrap().chains.len(),
            joined_channels: CHANNELS.to_vec(),
            uptime_ms: ELAPSED_TIME.lock().unwrap().elapsed().as_millis(),
        },
    })
}
