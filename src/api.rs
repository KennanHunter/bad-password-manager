use rocket::{get, routes};

#[get("/<id>")]
pub fn fetch_vault(id: u128) -> String {
    format!("Hello, {}!", id)
}
