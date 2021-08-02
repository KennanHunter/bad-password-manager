use rocket::{routes, Route};

mod new;

pub fn get_routes() -> Vec<Route> {
    routes![new::add_user]
}
