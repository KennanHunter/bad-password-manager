use crate::credentials::*;
use crate::hashes;
// mod credentials;

pub fn newuser() {
    let stuff = read_creds();
    println!("{:#?}", hashes::get_hashes(&stuff));
}
