use clap::{clap_app, crate_version};
use open;

mod credentials;
mod hashes;
mod newuser;

fn main() {
    let version = crate_version!();

    let matches = clap_app!(myapp =>
        (version: version)
        (author: "Kennan Hunter <kennanhunter5@gmail.com>")
        (about: "Bad-Password-Manager CLI")
        (@subcommand login =>
            (about: "logs in to server")
            (version: version)
            (@arg server: -s --server "log into non standard server")
        )
        (@subcommand newuser =>
            (about: "create a new user")
            (version: version)
        )
        (@subcommand rolley =>
            (about: "rolley")
            (version: version)
        )
    )
    .get_matches();

    match matches.subcommand_name() {
        Some("login") => println!("login"),
        Some("newuser") => newuser::newuser(),
        Some("rolley") => rolley(),
        Some("get") => println!("getting password"),
        None => println!("No subcommand was used"),
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }

    println!("{:#?}", matches);
    println!("this doesn't work btw");
}

pub fn rolley() {
    open::that(
        "https://media.discordapp.net/attachments/824500230119489566/872525002770120714/image0.png",
    )
    .unwrap();
}
