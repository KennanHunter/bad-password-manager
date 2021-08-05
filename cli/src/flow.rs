use crate::credentials::Credentials;

use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher},
    Pbkdf2,
};

pub fn get_email_password_hash(creds: &Credentials) -> PasswordHash {
    // Pbkdf2.hash_password_simple(&creds.Password[..], &creds.Username).unwrap().to_string()
    Pbkdf2
        .hash_password_simple(&creds.Password.as_bytes(), &creds.Username)
        .unwrap()
}
