use rand::rngs::OsRng;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, PasswordHasher as _};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    hash
}

pub fn verify_password(hashed: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
