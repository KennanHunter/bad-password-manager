#![feature(decl_macro)]

mod vault;
mod user;

use rocket;

fn main() {
    rocket::ignite()
        // .mount("/", StaticFiles::from("/app/")) // switched to handling it with nginx
        .mount("/api/vault", vault::get_routes())
        .mount("/api/user", user::get_routes())

        .launch();
}
