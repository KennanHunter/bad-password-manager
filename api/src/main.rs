#![feature(decl_macro)]

mod vault;

use rocket::{get, routes};

fn main() {
    rocket::ignite()
        // .mount("/", StaticFiles::from("/app/")) // switched to handling it with nginx
        .mount("/", vault::get_routes())
        .launch();
}
