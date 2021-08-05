use std::io::{self, Write};

#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn read_creds() -> Credentials {
    print!("Email: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    println!("Your username is {}", username.trim());

    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    println!("Your password is {}", pass.trim());

    Credentials {
        username: username.trim().to_string(),
        password: pass.trim().to_string(),
    }
}
