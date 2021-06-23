use rocket::{get, routes};

#[get("/<id>")]
pub fn fetch_vault(id: String) -> String {
    format!("Hello, {}!", id)
}
