use crate::credentials::Credentials;

use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher},
    Pbkdf2,
};

pub struct Hashes {
    pub email_pass: PasswordHash,
    pub master_pass: PasswordHash,
}

pub fn get_hashes(creds: &Credentials) -> PasswordHash {
    let email_pass_hash = Pbkdf2
        .hash_password_simple(&creds.password.as_bytes(), &creds.username)
        .unwrap();
    Pbkdf2
        .hash_password_simple(email_pass_hash.hash.unwrap().as_bytes(), &creds.password)
        .unwrap()
}
