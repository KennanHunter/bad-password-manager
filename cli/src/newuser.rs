use crate::credentials::*;
use crate::flow;
// mod credentials;

pub fn newuser() {
    let stuff = read_creds();
    println!("{:#?}", flow::get_email_password_hash(&stuff));
}
