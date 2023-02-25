use crate::CHAINS;

#[get("/gen?<message>")]
pub fn gen_text(message: String) -> String {
    CHAINS.lock().unwrap().generate_text(&message)
}
