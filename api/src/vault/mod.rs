use rocket::{get, routes, Route};

pub fn get_routes() -> Vec<Route> {
    routes![fetch_vault]
}

#[get("/<id>")]
pub fn fetch_vault(id: String) -> String {
    format!("Hello, {}!", id)
}
