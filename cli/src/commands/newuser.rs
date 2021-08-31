use crate::util::credentials::*;
use crate::util;
// mod credentials;

///Generates a new user
pub fn newuser() {
	let stuff = read_creds();
	println!("{:#?}", util::hashes::get_hashes(&stuff));
}
