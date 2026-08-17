mod utils;

use argon2::{Argon2, password_hash::{SaltString}, PasswordHasher};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, argo2!");
}

#[wasm_bindgen]
pub fn argon2id(password: String, salt: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::from_b64(&salt).unwrap();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    password_hash
}