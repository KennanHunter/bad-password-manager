use crate::credentials::*;
use crate::hashes;
// mod credentials;

///Generates a new user
pub fn newuser() {
    let stuff = read_creds();
    println!("{:#?}", hashes::get_hashes(&stuff));
}
