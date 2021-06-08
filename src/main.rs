#![feature(decl_macro)]

use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;
use bad_pass_manager::fetch_vault;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("/app/"))
        .mount("/api/", routes![fetch_vault])
        .launch();
}
