#![feature(decl_macro)]

mod vault;
mod user;

use rocket;

fn main() {
    rocket::ignite()
        // .mount("/", StaticFiles::from("/app/")) // switched to handling it with nginx
        .mount("/vault", vault::get_routes())
        .mount("/user", user::get_routes())

        .launch();
}
