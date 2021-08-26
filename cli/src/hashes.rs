use crate::credentials::Credentials;

use pbkdf2::{
	password_hash::{PasswordHash, PasswordHasher},
	Pbkdf2,
};

#[derive(Debug)]
pub struct Hashes<'a> {
	pub email_pass: PasswordHash<'a>,
	pub master_pass: PasswordHash<'a>,
}

/// Gets all the hashes from user credentials present in [`Hashes`]
/// This is used in order to authenticate with the api
pub fn get_hashes(creds: &Credentials) -> Hashes {
	let email_pass_hash = Pbkdf2
		.hash_password_simple(creds.password.as_bytes(), &creds.username)
		.unwrap();
	let master_pass_hash = Pbkdf2
		.hash_password_simple(email_pass_hash.hash.unwrap().as_bytes(), &creds.password)
		.unwrap();
	Hashes {
		email_pass: email_pass_hash,
		master_pass: master_pass_hash,
	}
}
