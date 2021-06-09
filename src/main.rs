#![feature(decl_macro)]

mod api;

use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("/app/"))
        .mount("/api/", routes![api::fetch_vault])
        .launch();
}
