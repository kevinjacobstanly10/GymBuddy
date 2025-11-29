use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

const SECRET: &[u8] = b"SUPER_SECRET_KEY_CHANGE_THIS";

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

pub fn generate_jwt(user_id: i64) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 60 * 60 * 24; // 24 hours

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
        .unwrap()
}

pub fn verify_jwt(token: &str) -> Option<i64> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default()
    );

    match decoded {
        Ok(c) => Some(c.claims.sub),
        Err(_) => None,
    }
}
