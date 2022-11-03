use chrono::DateTime;
use chrono::Local;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub created_at: DateTime<Local>,
    pub email: String,
    password: String,
    pub active: bool,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        dbg!(User {
            username,
            email,
            password: Self::hash_string(password),
            active: true,
            created_at: Local::now(),
        })
    }

    pub fn auth(&self, username: &String, other_password: &String) -> bool {
        if self.username == username.trim()
            && self.password == Self::hash_string(other_password.trim().to_string())
        {
            return true;
        } else {
            return false;
        }
    }

    pub fn hash_string(value: String) -> String {
        let mcrypt = new_magic_crypt!(std::env::var("SECRET").expect("SECRET must be set."), 256);

        mcrypt.encrypt_str_to_base64(value)
    }
}
