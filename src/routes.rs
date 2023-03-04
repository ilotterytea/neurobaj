use diesel::{QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::{chains::generate_text, establish_connection, models::Chain, ELAPSED_TIME};

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
    generate_text(&mut establish_connection(), message.as_str())
}

#[get("/status")]
pub fn status() -> Json<Response<Status>> {
    let conn = &mut establish_connection();

    let total_chains = crate::schema::chains::dsl::chains
        .load::<Chain>(conn)
        .unwrap();
    let joined_channels = crate::schema::channels::dsl::channels
        .select(crate::schema::channels::dsl::alias_id)
        .load::<String>(conn)
        .unwrap();

    Json(Response {
        status: 200,
        data: Status {
            total_chains: total_chains.len(),
            joined_channels,
            uptime_ms: ELAPSED_TIME.lock().unwrap().elapsed().as_millis(),
        },
    })
}
